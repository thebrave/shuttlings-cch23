mod day_0;
mod day_1;
mod day_4;

use crate::day_0::{day0_error, day0_hello};
use crate::day_1::day1_cube;
use crate::day_4::{day4_contest, day4_strength};
use actix_web::web::PathConfig;
use actix_web::{error, get, web, web::ServiceConfig, HttpRequest, HttpResponse};
use shuttle_actix_web::ShuttleActixWeb;
use tracing::error;

async fn default_handler(req: HttpRequest) -> HttpResponse {
    error!("> default {:?} {:?}", req.method(), req.path());
    HttpResponse::BadRequest().finish()
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        // Day -1
        cfg.service(day0_hello);
        cfg.service(day0_error);

        // Day 1
        cfg.service(day1_cube);

        // Day 4
        cfg.service(day4_strength);
        cfg.service(day4_contest);

        // Default handler (for debug)
        cfg.default_service(web::route().to(default_handler));
        cfg.app_data(PathConfig::default().error_handler(|err, req| {
            error!("! {} failed because {}", req.uri().path(), err);
            error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
        }));
    };

    Ok(config.into())
}
