use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Error, PgPool};
use tracing::{error, info};

#[derive(sqlx::FromRow, Deserialize)]
struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

#[derive(Serialize)]
struct Total {
    total: i64,
}

#[derive(Serialize)]
struct Popular {
    popular: String,
}

#[get("/13/sql")]
async fn day13_sql(pool: web::Data<PgPool>) -> HttpResponse {
    let db = pool.get_ref();
    let result: (i32,) = sqlx::query_as("SELECT 20231213")
        .fetch_one(db)
        .await
        .unwrap();

    HttpResponse::Ok().body(result.0.to_string())
}

#[post("/13/reset")]
async fn day13_reset(pool: web::Data<PgPool>) -> HttpResponse {
    info!("> reset");
    let db = pool.get_ref();
    match sqlx::query("DELETE FROM orders").execute(db).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error while reset: {:?}", err))
        }
    }
}

#[post("/13/orders")]
async fn day13_orders(pool: web::Data<PgPool>, info: web::Json<Vec<Order>>) -> HttpResponse {
    info!("> orders");
    let db = pool.get_ref();
    for order in info.iter() {
        match sqlx::query(
            "INSERT INTO orders (id, region_id, gift_name, quantity) VALUES($1, $2, $3, $4)",
        )
        .bind(order.id)
        .bind(order.region_id)
        .bind(order.gift_name.clone())
        .bind(order.quantity)
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

#[get("/13/orders/total")]
async fn day13_total(pool: web::Data<PgPool>) -> HttpResponse {
    info!("> total");
    let db = pool.get_ref();
    match sqlx::query_as::<_, (i64,)>("SELECT SUM(quantity) FROM orders")
        .fetch_one(db)
        .await
    {
        Ok(sum) => HttpResponse::Ok().json(Total { total: sum.0 }),
        Err(err) => {
            error!("! Error while querying draft: {:?}", err);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

#[get("/13/orders/popular")]
async fn day13_popular(pool: web::Data<PgPool>) -> HttpResponse {
    info!("> popular");
    let db = pool.get_ref();
    match sqlx::query_as::<_, (String, i64)>(
        "SELECT gift_name, SUM(quantity) AS count FROM orders GROUP BY gift_name ORDER BY count DESC",
    )
    .fetch_one(db)
    .await
    {
        Ok(sum) => {
            info!("= Most popular: {}({})", sum.0, sum.1);
            HttpResponse::Ok().json(Popular { popular: sum.0 }) },
        Err(err) => match err {
            Error::RowNotFound => HttpResponse::Ok().json(json!({
        "popular": null})),
            _ => {
                error!("! Error while querying draft: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        },
    }
}
