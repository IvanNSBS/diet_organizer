use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router, extract::Query, extract::Path, Extension,
};
use serde::{Deserialize, Serialize};
use tracing_subscriber::layer;
use std::{net::SocketAddr, sync::Mutex};

use crate::{db_food::Recipe, db_service::{DBAdapter, SqliteAdapter}};

pub struct WebServer;

impl WebServer{
    pub async fn start(&self, port: u16, host: &str) {
        tracing_subscriber::fmt::init();
        let app = self.create_routes();

        let url = format!("{}:{}", host, port);
        let address: SocketAddr = url.parse().expect("Unable to parse socket address");

        println!("Starting server at {}", address.to_string());
        axum::Server::bind(&address)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }

    fn create_routes(&self) -> Router {
        // build our application with a route
        let app = Router::new()
            .route("/", get(root))
            .route("/all_foods", get(get_foods))
            .route("/get_food/:food_id", get(get_foods_by_id));
    
        return app;
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_foods() -> Json<Recipe> {
    let r = Recipe {
        id: 0,
        uuid: "asdasd".to_string(),
        name: "test".to_string(),
        foods: Vec::new()
    };

    return Json(r);
}

async fn get_foods_by_id(Path(id):Path<i32> ) -> Json<Recipe> {
    let r = Recipe {
        id: id,
        uuid: "asdasd".to_string(),
        name: "test".to_string(),
        foods: Vec::new()
    };

    return Json(r);
}