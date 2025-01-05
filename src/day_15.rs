use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
struct Input {
    input: String,
}

#[post("/15/nice")]
async fn day15_nice(info: web::Json<Input>) -> HttpResponse {
    info!("> nice");
    HttpResponse::InternalServerError().finish()
}

#[post("/15/game")]
async fn day15_game(info: web::Json<Input>) -> HttpResponse {
    info!("> game");
    HttpResponse::InternalServerError().finish()
}