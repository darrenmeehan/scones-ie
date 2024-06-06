use axum::{routing::get, Router};
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use tower_http::services::ServeDir;

mod configuration;
pub mod database;
mod github;
mod handlers;
pub mod models;
pub mod relme;
pub mod schema;
use crate::configuration::get_configuration;
use crate::database::connect;
use crate::github::callback_handler;
use crate::handlers::{
    authorization_handler, client_handler, error_handler, healthcheck_handler, metadata_handler,
    token_handler,
};

pub async fn run() {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app(connect().await)).await.unwrap();
}

pub fn app(pool: Pool<Manager<PgConnection>>) -> Router {
    Router::new()
        .route("/healthcheck", get(healthcheck_handler))
        .nest_service("/", ServeDir::new("static"))
        .route(
            "/.well-known/oauth-authorization-server",
            get(metadata_handler),
        )
        .route("/api/metadata", get(metadata_handler))
        .route("/api/auth", get(authorization_handler))
        .route("/api/callback", get(callback_handler))
        .route("/api/token", get(token_handler))
        .route("/api/client", get(client_handler))
        .route("/api/error", get(error_handler))
        .with_state(pool)
}
