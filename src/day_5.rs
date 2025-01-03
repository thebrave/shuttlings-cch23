use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
struct Info {
    #[serde(default)]
    offset: usize,
    limit: Option<usize>,
    split: Option<usize>,
}

#[post("/5")]
async fn day5_page(info: web::Query<Info>, data: web::Json<Vec<String>>) -> HttpResponse {
    let limit = match info.limit {
        Some(limit) => limit,
        None => data.len(),
    };
    let end = std::cmp::min(info.offset + limit, data.len());

    match info.split {
        None => HttpResponse::Ok().json(data[info.offset..end].to_vec()),
        Some(0) => {
            let out: Vec<String> = Vec::new();
            HttpResponse::Ok().json(&out)
        }
        Some(split) => {
            let count = end - info.offset;
            let mut out = Vec::new();

            info!(
                "= offset:{}, limit:{}, split:{}, end:{}, r:{}",
                info.offset,
                limit,
                split,
                end,
                (count / split)
            );

            for i in 0..(count as f32 / split as f32).ceil() as usize {
                let b = info.offset + (i * split);
                info!(
                    "= b:{}/{}, {}->{}",
                    i,
                    (count / split),
                    b,
                    std::cmp::min(data.len(), b + split)
                );
                out.push(data[b..std::cmp::min(data.len(), b + split)].to_vec());
            }

            HttpResponse::Ok().json(&out)
        }
    }
}
