mod db_food;
mod db_service;

use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use crate::db_service::{SqliteAdapter, DBAdapter};

#[tokio::main]
async fn main() {
    // initialize tracing

    let mut db_conn = SqliteAdapter::new();
    db_conn.start();

    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root));

    let port = 3000;
    let url = SocketAddr::from(([127,0,0,1], port));

    println!("Starting server at {}", url.to_string());
    // run our app with hyper, listening globally on port 3000
    axum::Server::bind(&url)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}