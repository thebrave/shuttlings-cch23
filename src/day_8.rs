use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("/8/weight/{pokedex}")]
async fn day8_weight(path: web::Path<u32>) -> HttpResponse {
    match reqwest::get(format!(
        "https://pokeapi.co/api/v2/pokemon/{}/",
        path.into_inner()
    ))
    .await
    {
        Ok(res) => match res.json::<serde_json::Value>().await {
            Ok(json) => {
                if json.is_object() && json.as_object().unwrap().contains_key("weight") {
                    let weight = json["weight"].as_f64().unwrap() / 10.0;
                    return HttpResponse::Ok().body(weight.to_string());
                }
                HttpResponse::BadRequest().finish()
            }
            Err(_) => HttpResponse::BadRequest().finish(),
        },
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[get("/8/drop/{pokedex}")]
async fn day8_drop(path: web::Path<u32>) -> HttpResponse {
    match reqwest::get(format!(
        "https://pokeapi.co/api/v2/pokemon/{}/",
        path.into_inner()
    ))
    .await
    {
        Ok(res) => match res.json::<serde_json::Value>().await {
            Ok(json) => {
                if json.is_object() && json.as_object().unwrap().contains_key("weight") {
                    let weight = (json["weight"].as_f64().unwrap() / 10.0)
                        * ((2.0 * 9.825 * 10.0) as f64).sqrt();
                    return HttpResponse::Ok().body(weight.to_string());
                }
                HttpResponse::BadRequest().finish()
            }
            Err(_) => HttpResponse::BadRequest().finish(),
        },
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
