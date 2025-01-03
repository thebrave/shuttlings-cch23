mod day_0;
mod day_1;
mod day_4;
mod day_5;
mod day_6;

use crate::day_0::{day0_error, day0_hello};
use crate::day_1::day1_cube;
use crate::day_4::{day4_contest, day4_strength};
use crate::day_5::day5_page;
use actix_web::web::PathConfig;
use actix_web::{error, web, web::ServiceConfig, HttpRequest, HttpResponse};
use day_6::day6_search;
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

        // Day 5
        cfg.service(day5_page);

        // Day 6
        cfg.service(day6_search);

        // Default handler (for debug)
        cfg.default_service(web::route().to(default_handler));
        cfg.app_data(PathConfig::default().error_handler(|err, req| {
            error!("! {} failed because {}", req.uri().path(), err);
            error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
        }));
    };

    Ok(config.into())
}
