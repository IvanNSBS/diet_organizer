use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

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
            .route("/", get(root));
    
        return app;
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}