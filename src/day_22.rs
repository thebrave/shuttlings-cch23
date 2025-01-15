use std::collections::HashSet;

use actix_web::{post, Error, HttpRequest, HttpResponse};
use tracing::{error, info};

#[post("/22/integers")]
async fn day22_integers(req: HttpRequest, stream: String) -> HttpResponse {
    info!("> {}", req.path());
    let mut books: HashSet<usize> = HashSet::new();
    for str in stream.split('\n') {
        if str.is_empty() {
            continue;
        }
        match str.parse() {
            Ok(number) => {
                if books.contains(&number) {
                    books.remove(&number);
                } else {
                    books.insert(number);
                }
            }
            Err(err) => {
                error!("! {} can't be parsed as an integer: {:?}", &str, err);
                return HttpResponse::BadRequest().finish();
            }
        }
    }
    match books.iter().last() {
        Some(number) => {
            info!("< unique number: {}", number);
            HttpResponse::Ok().body("🎁".repeat(*number))
        }
        None => {
            info!("< no unique number");
            HttpResponse::NotFound().finish()
        }
    }
}

#[post("/22/rocket")]
async fn day22_rocket(req: HttpRequest, stream: String) -> Result<HttpResponse, Error> {
    info!("> {}", req.path());
    Ok(HttpResponse::Ok().finish())
}
