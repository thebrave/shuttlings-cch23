use actix_web::{get, HttpRequest, HttpResponse};

#[get("/")]
async fn day0_hello() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/-1/error")]
async fn day0_error(req: HttpRequest) -> HttpResponse {
    HttpResponse::InternalServerError().finish()
}