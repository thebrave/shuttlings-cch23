use actix_web::{get, web, HttpRequest, HttpResponse};
use s2::cellid::CellID;
use tracing::info;

#[get("/21/coords/{binary}")]
async fn day21_coords(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    info!("> {}", req.path());
    match u64::from_str_radix(&path, 2) {
        Ok(id) => {
            let cell = CellID(id);
        }
        Err(_) => todo!(),
    }
    HttpResponse::BadRequest().finish()
}

#[get("/21/country/{binary}")]
async fn day21_country(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    info!("> {}", req.path());
    HttpResponse::BadRequest().finish()
}
