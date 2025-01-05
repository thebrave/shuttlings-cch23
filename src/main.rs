mod day_0;
mod day_1;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

use actix_files::Files;
use actix_web::web::PathConfig;
use actix_web::{error, web, web::ServiceConfig, HttpRequest, HttpResponse};
use day_0::{day0_error, day0_hello};
use day_1::day1_cube;
use day_11::day11_redpixels;
use day_12::{day12_load, day12_lsb, day12_save, day12_ulid, Day12State};
use day_13::{day13_orders, day13_popular, day13_reset, day13_sql, day13_total};
use day_14::{day14_safe, day14_unsafe};
use day_4::{day4_contest, day4_strength};
use day_5::day5_page;
use day_6::day6_search;
use day_7::{day7_bake, day7_decode};
use day_8::{day8_drop, day8_weight};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Mutex;
use tracing::error;

async fn default_handler(req: HttpRequest) -> HttpResponse {
    error!("> default {:?} {:?}", req.method(), req.path());
    HttpResponse::BadRequest().finish()
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let day_12_state = web::Data::new(Mutex::new(Day12State {
        store: HashMap::new(),
    }));

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

        // Day 12
        cfg.service(day12_save);
        cfg.service(day12_load);
        cfg.service(day12_ulid);
        cfg.service(day12_lsb);

        // Day 13
        cfg.service(day13_sql);
        cfg.service(day13_reset);
        cfg.service(day13_orders);
        cfg.service(day13_total);
        cfg.service(day13_popular);

        // Day 14
        cfg.service(day14_safe);
        cfg.service(day14_unsafe);

        // App states
        cfg.app_data(day_12_state.clone());
        cfg.app_data(web::Data::new(pool));

        // Default handler (for debug)
        cfg.default_service(web::route().to(default_handler));
        cfg.app_data(PathConfig::default().error_handler(|err, req| {
            error!("! {} failed because {}", req.uri().path(), err);
            error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
        }));
    };

    Ok(config.into())
}
