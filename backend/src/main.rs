extern crate core;

use crate::server::health_check;
use axum::routing::get;
use axum::Router;
use serde::Serialize;

mod parsers;
mod rationals;
mod utils;
mod document;
mod solvers;
mod dto;
mod server;



#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/health", get(health_check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}