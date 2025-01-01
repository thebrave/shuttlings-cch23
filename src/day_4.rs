use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use std::collections::HashMap;
use std::iter::Map;
use tracing::callsite::rebuild_interest_cache;

#[derive(Deserialize)]
struct Info {
    name: String,
    strength: u32,
}

#[derive(Deserialize)]
struct Contest {
    name: String,
    strength: i32,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename(
        serialize = "cAnD13s_3ATeN-yesT3rdAy",
        deserialize = "cAnD13s_3ATeN-yesT3rdAy"
    ))]
    candies: i32,
}

struct Value<T> {
    name: String,
    value: T,
}

/// extract `Info` using serde
#[post("/4/strength")]
async fn day4_strength(info: web::Json<Vec<Info>>) -> HttpResponse {
    let mut sum = 0;
    info.into_inner().iter().for_each(|i| {
        sum += i.strength;
    });
    HttpResponse::Ok().body(sum.to_string())
}

struct Winners {
    fastest: Value<f32>,
    tallest: Value<i32>,
    magician: Value<i32>,
    consumer: Value<i32>,
    favorite_food: String,
    strength: i32,
    antler_width: i32,
}

#[post("/4/contest")]
async fn day4_contest(info: web::Json<Vec<Contest>>) -> HttpResponse {
    let mut winners: Winners = Winners {
        fastest: Value {
            name: "".to_string(),
            value: 0f32,
        },
        tallest: Value {
            name: "".to_string(),
            value: 0,
        },
        magician: Value {
            name: "".to_string(),
            value: 0,
        },
        consumer: Value {
            name: "".to_string(),
            value: 0,
        },
        favorite_food: "".to_string(),
        strength: 0,
        antler_width: 0,
    };

    for reindeer in info.into_inner() {
        if reindeer.snow_magic_power > winners.magician.value {
            winners.magician.value = reindeer.snow_magic_power;
            winners.magician.name = reindeer.name.clone();
        }
        if reindeer.speed > winners.fastest.value {
            winners.fastest.value = reindeer.speed;
            winners.strength = reindeer.strength;
            winners.fastest.name = reindeer.name.clone();
        }
        if reindeer.height > winners.tallest.value {
            winners.tallest.value = reindeer.height;
            winners.antler_width = reindeer.antler_width;
            winners.tallest.name = reindeer.name.clone();
        }
        if reindeer.candies > winners.consumer.value {
            winners.consumer.value = reindeer.candies;
            winners.favorite_food = reindeer.favorite_food.clone();
            winners.consumer.name = reindeer.name.clone();
        }
    }
    let mut out = HashMap::new();
    out.insert(
        "magician",
        format!(
            "{} could blast you away with a snow magic power of {}",
            winners.magician.name, winners.magician.value
        ),
    );
    out.insert(
        "fastest",
        format!(
            "Speeding past the finish line with a strength of {} is {}",
            winners.strength, winners.fastest.name
        ),
    );
    out.insert(
        "tallest",
        format!(
            "{} is standing tall with his {} cm wide antlers",
            winners.tallest.name, winners.antler_width
        ),
    );
    out.insert(
        "consumer",
        format!(
            "{} ate lots of candies, but also some {}",
            winners.consumer.name, winners.favorite_food
        ),
    );
    HttpResponse::Ok().json(&out)
}
