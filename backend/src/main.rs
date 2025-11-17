extern crate core;

use crate::server::{handle_panic, health_check, panic, solve_primary_simplex, verify_mps_model};
use axum::routing::{get, post, put};
use tower_http::catch_panic::CatchPanicLayer;
use axum::Router;
use log::info;

mod parsers;
mod rationals;
mod utils;
mod document;
mod solvers;
mod dto;
mod server;



#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting Simpler backend server.");

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/solve/primary-simplex", post(solve_primary_simplex))
        .route("/api/verify/mps", put(verify_mps_model))
        .route("/api/panic", get(panic))
        .layer(CatchPanicLayer::custom(handle_panic));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}