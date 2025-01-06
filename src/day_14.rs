use actix_web::{post, web, HttpResponse};
use htmlize::escape_attribute;
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
struct Content {
    content: String,
}

#[post("/14/unsafe")]
async fn day14_unsafe(info: web::Json<Content>) -> HttpResponse {
    info!("> unsafe");
    let text = format!(
        r#"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>"#,
        info.content
    );
    HttpResponse::Ok().body(text)
}

#[post("/14/safe")]
async fn day14_safe(info: web::Json<Content>) -> HttpResponse {
    info!("> safe");
    let safe_n = escape_attribute(&info.content);
    let text = format!(
        r#"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>"#,
        safe_n
    );
    HttpResponse::Ok().body(text)
}
