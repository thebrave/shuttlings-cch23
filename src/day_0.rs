use actix_web::{get, HttpResponse};

#[get("/")]
async fn day0_hello() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/-1/error")]
async fn day0_error() -> HttpResponse {
    HttpResponse::InternalServerError().finish()
}
