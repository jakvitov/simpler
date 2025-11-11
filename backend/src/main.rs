extern crate core;

use crate::server::{health_check, solve_primary_simplex};
use axum::routing::{get, post};
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
        .route("/api/health", get(health_check))
        .route("/api/solve/primary-simplex", post(solve_primary_simplex));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}