use axum::{routing::get, routing::post, Router};
use tower_http::services::ServeDir;
use tracing::{event, Level};

mod configuration;
pub mod database;
mod github;
mod handlers;
pub mod models;
pub mod schema;
use crate::configuration::get_configuration;

use crate::handlers::{authorization_handler, error_handler, healthcheck_handler, upload_handler};

use crate::github::callback_handler;

pub async fn run() {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    event!(
        Level::INFO,
        "listening on {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app()).await.unwrap();
}

pub fn app() -> Router {
    Router::new()
        .route("/healthcheck", get(healthcheck_handler))
        .nest_service("/", ServeDir::new("static"))
        .route("/upload", post(upload_handler))
        .route("/api/auth", get(authorization_handler))
        .route("/api/callback", get(callback_handler))
        .route("/api/error", get(error_handler))
}
