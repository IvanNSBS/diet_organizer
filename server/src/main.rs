mod db_food;
mod db_service;
mod web_server;

use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use web_server::WebServer;
use std::net::SocketAddr;

use crate::db_service::{SqliteAdapter, DBAdapter};

#[tokio::main]
async fn main() {
    // initialize tracing

    let mut db_conn = SqliteAdapter::new();
    db_conn.start();

    let server = WebServer;
    server.start(3000, "127.0.0.1").await;
}