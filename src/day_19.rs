use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use actix_web::{get, post, rt, web, Error, HttpRequest, HttpResponse};
use actix_ws::{AggregatedMessage, Session};
use futures_util::StreamExt as _;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::info;

#[derive(Deserialize)]
struct InputMessage {
    message: String,
}

#[derive(Serialize)]
struct BroadcastMessage {
    user: String,
    message: String,
}

pub struct Day19State {
    pub(crate) views: usize,
    pub(crate) sessions: HashMap<usize, HashSet<Session>>,
}

#[get("/19/ws/ping")]
async fn day19_ping(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    // start task but don't wait for it
    rt::spawn(async move {
        let mut is_started = false;
        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => match text.as_ref() {
                    "serve" => is_started = true,
                    "ping" => {
                        if is_started {
                            session.text("pong").await.unwrap();
                        }
                    }
                    _ => {}
                },

                Ok(AggregatedMessage::Ping(msg)) => {
                    // respond to PING frame with PONG frame
                    session.pong(&msg).await.unwrap();
                }

                _ => {}
            }
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
}

#[post("/19/reset")]
async fn day19_reset(data: web::Data<Mutex<Day19State>>, req: HttpRequest) -> HttpResponse {
    info!("> {}", req.path());

    match data.lock() {
        Ok(mut state) => {
            state.views = 0;
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/19/views")]
async fn day19_views(
    req: HttpRequest
) -> HttpResponse {
    info!("> {}", req.path());
    HttpResponse::InternalServerError().finish()
}

#[get("/19/ws/room/{number}/user/{string}")]
async fn day19_tweet(
    data: web::Data<Mutex<Day19State>>,
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<(usize, String)>,
) -> Result<HttpResponse, Error> {
    info!("> {}", req.path());
    let (number, user) = path.into_inner();
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    let mut state = match data.lock() {
        Ok(state) => state,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    if !state.sessions.contains_key(&number) {
        state.sessions.insert(number, HashSet::new());
    }

    drop(state);

    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    let cmd: InputMessage = serde_json::from_str(&text).unwrap();
                    let mut state = match data.lock() {
                        Ok(state) => state,
                        Err(_) => return,
                    };
                    state.views += 1;
                    let bm = BroadcastMessage {
                        user: user.clone(),
                        message: cmd.message,
                    };
                    let out = serde_json::to_string(&bm).unwrap();

                    match state.sessions.get(&number) {
                        Some(sessions) => {
                            for session in sessions {
                                session.clone().text(out.clone()).await.unwrap();
                            }
                        }
                        _ => {}
                    };
                }
                Ok(AggregatedMessage::Ping(msg)) => {
                    // respond to PING frame with PONG frame
                    session.pong(&msg).await.unwrap();
                }

                _ => {}
            }
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
}
