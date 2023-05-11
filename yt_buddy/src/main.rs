use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());

    // region: axum::Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> LISTENING on {}\n", addr);
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: axum::Server
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./public")))
}

// region: Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handle_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
// e.g., `/hello?name=Spiros`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("<h1>Hello, <strong>{name}!!!</strong></h1>"))
}

// e,g., `/hello/Spiros`
async fn handle_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handle_hello2 - {name:?}", "HANDLER");

    Html(format!("<h1>Hello, <strong>{name}!!!</strong></h1>"))
}
// endregion: Handler Hello
