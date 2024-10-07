#![allow(unused)]

pub use self::error::{Error, Result};

use std::net::SocketAddr;

use axum::{extract::{Path, Query}, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Router};
use model::ModelController;
use serde::Deserialize;
use tower_cookies::{CookieManagerLayer};
use tower_http::services::ServeDir;

mod error;
mod web;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize ModelController
    let mc = ModelController::new().await?;

    let routes_all = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .nest("/api", web::routes_tickets::routes(mc.clone()))
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("--> LISTENING ON ADDR {addr}\n");

    axum::Server::bind(&addr).serve(
        routes_all.into_make_service()
    ).await.unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("--> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();

    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
    .route("/hello", get(handler_hello),)
    .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g., /hello?name=fede
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");

    Html(format!("Hello {name}!"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello {name}!"))
}