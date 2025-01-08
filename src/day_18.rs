use actix_web::{get, post, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Error, PgPool};
use tracing::{error, info};

#[derive(sqlx::FromRow, Deserialize)]
struct Region {
    id: i32,
    name: String,
}

#[derive(sqlx::FromRow, Serialize)]
struct Total {
    region: String,
    total: i64,
}

#[derive(Serialize)]
struct TopList {
    region: String,
    top_gifts: Vec<String>,
}

#[post("/18/reset")]
async fn day18_reset(pool: web::Data<PgPool>, req: HttpRequest) -> HttpResponse {
    info!("> {}", req.path());
    let db = pool.get_ref();
    match sqlx::query("DELETE FROM orders").execute(db).await {
        Ok(_) => match sqlx::query("DELETE FROM regions").execute(db).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => HttpResponse::InternalServerError()
                .body(format!("Error while reset regions: {:?}", err)),
        },
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error while reset orders: {:?}", err))
        }
    }
}

#[post("/18/regions")]
async fn day18_region(
    pool: web::Data<PgPool>,
    info: web::Json<Vec<Region>>,
    req: HttpRequest,
) -> HttpResponse {
    info!("> {}", req.path());
    let db = pool.get_ref();
    for order in info.iter() {
        match sqlx::query("INSERT INTO regions (id, name) VALUES($1, $2)")
            .bind(order.id)
            .bind(&order.name)
            .execute(db)
            .await
        {
            Ok(_) => {}
            Err(err) => {
                error!("! Error while querying draft: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        }
    }

    HttpResponse::Ok().finish()
}

#[get("/18/regions/top_list/{number}")]
async fn day18_total(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<i32>,
) -> HttpResponse {
    info!("> {}", req.path());
    let db = pool.get_ref();
    let limit = path.into_inner();
    let mut out: Vec<TopList> = Vec::new();
    match sqlx::query_as::<_, (i32, String)>("SELECT id, name FROM regions")
        .fetch_all(db)
        .await
    {
        Ok(sum) => {
            for (id, name) in sum {
                match sqlx::query_as::<_, (String, )>("SELECT gift_name FROM orders WHERE region_id = $1 GROUP BY gift_name ORDER BY SUM(quantity) LIMIT $2")
                    .bind(id)
                    .bind(limit)
                    .fetch_all(db)
                    .await
                {
                    Ok(top) => {
                        let mut top_gifts:Vec<String>= Vec::new();
                        top.iter().for_each(|gift| {top_gifts.push(gift.0.clone())});
                        out.push(TopList{ region: name, top_gifts })
                    }
                    Err(err) => match err {
                        sqlx::Error::RowNotFound => {
                            let mut top_gifts:Vec<String>= Vec::new();
                            out.push(TopList{ region: name, top_gifts })},
                        _ => {
                            error!("! Error while querying draft: {:?}", err);
                            return HttpResponse::InternalServerError().finish();
                        }

                    }
                }
            }
            HttpResponse::Ok().json(out)
        }
        Err(err) => match err {
            sqlx::Error::RowNotFound => {
                let res: Vec<Total> = Vec::new();
                HttpResponse::Ok().json(res)
            }
            _ => {
                error!("! Error while querying draft: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        },
    }
}

#[get("/18/regions/total")]
async fn day18_toplist(pool: web::Data<PgPool>, req: HttpRequest) -> HttpResponse {
    info!("> {}", req.path());
    let db = pool.get_ref();
    match sqlx::query_as::<_, Total>(
        r#"SELECT r.name as region, SUM(quantity) as total
FROM orders o
JOIN regions r ON o.region_id = r.id
GROUP BY r.name ORDER BY name"#,
    )
    .fetch_all(db)
    .await
    {
        Ok(sum) => HttpResponse::Ok().json(sum),
        Err(err) => match err {
            sqlx::Error::RowNotFound => {
                let res: Vec<Total> = Vec::new();
                HttpResponse::Ok().json(res)
            }
            _ => {
                error!("! Error while querying draft: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        },
    }
}
