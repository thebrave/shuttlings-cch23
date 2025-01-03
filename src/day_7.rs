use std::collections::HashMap;

use actix_web::{get, HttpRequest, HttpResponse};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Deserialize)]
struct Recipe {
    recipe: HashMap<String, usize>,
    pantry: HashMap<String, usize>,
}

#[derive(Serialize, Debug)]
struct Output {
    cookies: usize,
    pantry: HashMap<String, usize>,
}

#[get("/7/decode")]
async fn day7_decode(req: HttpRequest) -> HttpResponse {
    match req.cookie("recipe") {
        Some(cookie) => match STANDARD.decode(cookie.value()) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(value) => HttpResponse::Ok().body(value),
                Err(e) => {
                    error!("! error while converting binary to string: {:?}", e);
                    HttpResponse::BadRequest().finish()
                }
            },
            Err(e) => {
                error!("! error while décoding base64: {:?}", e);
                HttpResponse::BadRequest().finish()
            }
        },
        None => {
            error!("! no cookie 'recipe' in request");
            HttpResponse::BadRequest().finish()
        }
    }
}

#[get("/7/bake")]
async fn day7_bake(req: HttpRequest) -> HttpResponse {
    info!("> day7_bake");
    let r: Recipe;
    match req.cookie("recipe") {
        Some(cookie) => match STANDARD.decode(cookie.value()) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(value) => match serde_json::from_str::<Recipe>(&value) {
                    Ok(recipe) => {
                        r = recipe;
                    }
                    Err(e) => {
                        error!("! error while deserializing recipe: {:?}", e);
                        return HttpResponse::BadRequest().finish();
                    }
                },
                Err(e) => {
                    error!("! error while converting binary to string: {:?}", e);
                    return HttpResponse::BadRequest().finish();
                }
            },
            Err(e) => {
                error!("! error while décoding base64: {:?}", e);
                return HttpResponse::BadRequest().finish();
            }
        },
        None => {
            error!("! no cookie 'recipe' in request");
            return HttpResponse::BadRequest().finish();
        }
    };

    let mut count: isize = -1;
    for (ingredient, quantity) in r.recipe.iter() {
        if *quantity == 0 {
            continue;
        }

        if !r.pantry.contains_key(ingredient) {
            info!("= pantry lack {}", ingredient);
            let out = Output {
                cookies: 0,
                pantry: r.pantry.clone(),
            };
            info!("< out:{:?}", out);
            return HttpResponse::Ok().json(out);
        }

        let cookables = r.pantry[ingredient] / quantity;
        if count == -1 || cookables < count as usize {
            count = cookables as isize;
        }
    }

    let mut out: Output = Output {
        cookies: count as usize,
        pantry: r.pantry.clone(),
    };

    for (ingredient, quantity) in r.recipe.iter() {
        if *quantity == 0 {
            continue;
        }

        let bef = out.pantry[ingredient];
        *(out.pantry.get_mut(ingredient).unwrap()) -= quantity * count as usize;
        info!("= {}: {} -> {}", ingredient, bef, out.pantry[ingredient]);
    }

    info!("< out:{:?}", out);
    HttpResponse::Ok().json(out)
}
