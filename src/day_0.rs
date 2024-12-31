#[get("/")]
async fn hello_world() -> HttpResponse {
    "Hello, bird!"
}
