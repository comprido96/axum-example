#![allow(unused)]

use std::net::SocketAddr;

use axum::{response::{Html, IntoResponse}, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(handler_hello),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("-> LISTENING ON ADDR {addr}\n");

    axum::Server::bind(&addr).serve(
        routes_hello.into_make_service()
    ).await.unwrap();
}

async fn handler_hello() -> impl IntoResponse {
    Html("Hello World!")
}