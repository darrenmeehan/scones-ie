use axum::{routing::get, Router};
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use tower_http::services::ServeFile;

pub mod database;
mod github;
mod handlers;
mod configuration;
pub mod relme;
use crate::database::connect;
use crate::github::callback_handler;
use crate::handlers::{
    authorization_handler, client_handler, error_handler, healthcheck_handler, metadata_handler,
    token_handler,
};
use crate::configuration::get_configuration;

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
        .route_service("/", ServeFile::new("static/index.html"))
        .route_service(
            "/admin/config.yml",
            ServeFile::new("static/admin/config.yml"),
        )
        .route_service(
            "/admin/index.html",
            ServeFile::new("static/admin/index.html"),
        )
        .route_service("/success", ServeFile::new("static/success.html"))
        .route_service("/logo.jpg", ServeFile::new("static/logo.jpg"))
        .route_service("/darren.jpg", ServeFile::new("static/darren.jpg"))
        .route_service("/login", ServeFile::new("static/login.html"))
        .route_service("/darren", ServeFile::new("static/profile.html"))
        .route(
            "/.well-known/oauth-authorization-server",
            get(metadata_handler),
        )
        .route("/metadata", get(metadata_handler))
        .route("/auth", get(authorization_handler))
        .route("/callback", get(callback_handler))
        .route("/token", get(token_handler))
        .route("/client", get(client_handler))
        .route("/error", get(error_handler))
        .with_state(pool)
}
