use actix_web::{get, web, HttpRequest, HttpResponse};
use tracing::info;

#[get("/1/{nums:.*}")]
async fn day1_cube(
    req: HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    info!("> day1_cube {}", req.uri().path());

    let n:Vec<i64> = path.into_inner().split('/')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let mut factor:i64 = 0;
    for i in n.iter() {
        factor ^= i;
    }

    match factor.checked_pow(3) {
        Some(num) => HttpResponse::Ok().body(num.to_string()),
        None => HttpResponse::InternalServerError().finish(),
    }
}