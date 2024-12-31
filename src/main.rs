mod day_0;
mod day_1;

use actix_web::{error, get, web, web::ServiceConfig, HttpRequest, HttpResponse};
use actix_web::web::PathConfig;
use shuttle_actix_web::ShuttleActixWeb;
use tracing::error;
use crate::day_0::{day0_error, day0_hello};
use crate::day_1::day1_cube;

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
        cfg.route()

        // Default handler (for debug)
        cfg.default_service(web::route().to(default_handler));
        cfg.app_data(PathConfig::default().error_handler(|err, req| {
            error!("! {} failed because {}", req.uri().path(), err);
            error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
        }));
    };

    Ok(config.into())
}
