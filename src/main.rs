mod day_0;
mod day_1;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_11;

use actix_files::Files;
use actix_web::web::PathConfig;
use actix_web::{error, web, web::ServiceConfig, HttpRequest, HttpResponse};
use day_0::{day0_error, day0_hello};
use day_1::day1_cube;
use day_4::{day4_contest, day4_strength};
use day_5::day5_page;
use day_6::day6_search;
use day_7::{day7_bake, day7_decode};
use day_8::{day8_drop, day8_weight};
use shuttle_actix_web::ShuttleActixWeb;
use tracing::error;
use crate::day_11::day11_redpixels;

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

        // Day 7
        cfg.service(day7_decode);
        cfg.service(day7_bake);

        // Day 8
        cfg.service(day8_weight);
        cfg.service(day8_drop);

        // Day 11
        cfg.service(Files::new("/11/assets", "assets"));
        cfg.service(day11_redpixels);

        // Default handler (for debug)
        cfg.default_service(web::route().to(default_handler));
        cfg.app_data(PathConfig::default().error_handler(|err, req| {
            error!("! {} failed because {}", req.uri().path(), err);
            error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
        }));
    };

    Ok(config.into())
}
