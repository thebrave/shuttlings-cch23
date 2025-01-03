use actix_web::{get, HttpRequest, HttpResponse};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use tracing::error;

#[get("/7/decode")]
async fn day7_decode(req: HttpRequest) -> HttpResponse {
    match req.cookie("recipe") {
        Some(cookie) => match STANDARD.decode(cookie.value()) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(value) => HttpResponse::Ok().body(value),
                Err(e) => {
                    error!("! error while converting binary to string: {:?}", e);
                    HttpResponse::BadRequest().finish()
                }
            },
            Err(e) => {
                error!("! error while décoding base64: {:?}", e);
                HttpResponse::BadRequest().finish()
            }
        },
        None => {
            error!("! no cookie 'recipe' in request");
            HttpResponse::BadRequest().finish()
        }
    }
}
