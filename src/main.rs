mod day_0;
mod day_1;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

use actix_files::Files;
use actix_web::web::{PathConfig, PayloadConfig};
use actix_web::{error, web, web::ServiceConfig, HttpRequest, HttpResponse};
use day_0::{day0_error, day0_hello};
use day_1::day1_cube;
use day_11::day11_redpixels;
use day_12::{day12_load, day12_lsb, day12_save, day12_ulid, Day12State};
use day_13::{day13_orders, day13_popular, day13_reset, day13_sql, day13_total};
use day_14::{day14_safe, day14_unsafe};
use day_15::{day15_game, day15_nice};
use day_18::{day18_region, day18_reset, day18_toplist, day18_total};
use day_19::{day19_ping, day19_reset, day19_tweet, day19_views, Day19State};
use day_20::{day20_number_files, day20_size_files};
use day_21::{day21_coords, day21_country};
use day_22::{day22_integers, day22_rocket};
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
    let day_19_state = web::Data::new(Mutex::new(Day19State {
        views: 0,
        sessions: HashMap::new(),
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

        // Day 15
        cfg.service(day15_nice);
        cfg.service(day15_game);

        // Day 18
        cfg.service(day18_reset);
        cfg.service(day18_region);
        cfg.service(day18_total);
        cfg.service(day18_toplist);

        // Day 19
        cfg.service(day19_ping);
        cfg.service(day19_reset);
        cfg.service(day19_views);
        cfg.service(day19_tweet);

        // Day 20
        cfg.service(day20_number_files);
        cfg.service(day20_size_files);

        // Day 22
        cfg.service(day22_integers);
        cfg.service(day22_rocket);

        // App states
        cfg.app_data(day_12_state.clone());
        cfg.app_data(web::Data::new(pool));
        cfg.app_data(day_19_state.clone());

        // Default handler (for debug)
        cfg.default_service(web::route().to(default_handler));
        cfg.app_data(PathConfig::default().error_handler(|err, req| {
            error!("! {} failed because {}", req.uri().path(), err);
            error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
        }));
        cfg.app_data(PayloadConfig::new(1_000_000));
    };

    Ok(config.into())
}
