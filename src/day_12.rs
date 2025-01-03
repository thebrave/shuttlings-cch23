use actix_web::{get, post, web, HttpResponse};
use chrono::{DateTime, Datelike, Utc};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use ulid::Ulid;
use uuid::Uuid;

pub struct Day12State {
    pub(crate) store: HashMap<String, u64>,
}

#[derive(Serialize)]
struct Lsb {
    #[serde(rename(serialize = "christmas eve", deserialize = "christmas eve"))]
    christmas: usize,
    weekday: usize,
    #[serde(rename(serialize = "in the future", deserialize = "in the future"))]
    future: usize,
    #[serde(rename(serialize = "LSB is 1", deserialize = "LSB is 1"))]
    lsb: usize,
}

#[post("/12/save/{string}")]
async fn day12_save(data: web::Data<Mutex<Day12State>>, path: web::Path<String>) -> HttpResponse {
    let mut state = data.lock().unwrap();
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            state.store.insert(path.into_inner(), n.as_secs());
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/12/load/{string}")]
async fn day12_load(data: web::Data<Mutex<Day12State>>, path: web::Path<String>) -> HttpResponse {
    let state = data.lock().unwrap();
    match state.store.get(&path.into_inner()) {
        Some(b) => match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(a) => HttpResponse::Ok().body((a.as_secs() - b).to_string()),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/12/ulids")]
async fn day12_ulid(path: web::Json<Vec<Ulid>>) -> HttpResponse {
    let mut out: Vec<Uuid> = Vec::new();
    for ulid in path.iter().rev() {
        out.push(Uuid::from_bytes(ulid.to_bytes()));
    }
    HttpResponse::Ok().json(&out)
}

#[post("/12/ulids/{weekday}")]
async fn day12_lsb(path: web::Json<Vec<Ulid>>, day: web::Path<u32>) -> HttpResponse {
    let mut out: Lsb = Lsb {
        christmas: 0,
        weekday: 0,
        future: 0,
        lsb: 0,
    };

    let day_of_week = day.into_inner();
    for ulid in path.iter() {
        let now = DateTime::<Utc>::from(SystemTime::now());
        let date = DateTime::<Utc>::from(ulid.datetime());
        if date.weekday().num_days_from_monday() == day_of_week {
            out.weekday += 1;
        }
        if date > now {
            out.future += 1;
        }
        if ulid.random() % 2 == 1 {
            out.lsb += 1;
        }
        if date.day() == 24 && date.month() == 12 {
            out.christmas += 1;
        }
    }

    HttpResponse::Ok().json(&out)
}
