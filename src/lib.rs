use std::collections::HashMap;

use axum::{routing::get, Router};
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use tower_http::services::ServeFile;

pub mod database;
mod github;
mod handlers;
pub mod relme;
use crate::database::connect;
use crate::github::callback_handler;
use crate::handlers::{authorization_handler, client_handler, healthcheck_handler, metadata_handler, token_handler};

pub async fn run() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app(connect().await)).await.unwrap();
}

pub fn build_auth_request(mut auth_endpoint: String, profile_uri: String) -> String {
    let mut params = HashMap::new();
    params.insert("response_type", "code".to_string());
    params.insert("client_uri", "https://scones.fly.dev/".to_string());
    params.insert("redirect_uri", "https://scones.fly.dev/client".to_string());
    params.insert("state", "changeme".to_string());
    params.insert("code_challenge", "123".to_string());
    params.insert("code_challenge_method", "S256".to_string());
    params.insert("me", profile_uri);

    let mut query = String::new();
    for (key, value) in params {
        query.push_str(&format!("{}={}&", key, value));
    }
    query.pop();

    auth_endpoint.push_str(&format!("?{}", query));
    auth_endpoint
}

pub async fn show_error() -> String {
    let message = "Could not find suitable RelMeAuth endpoint".to_string();
    format!("Something went wrong: {}", message)
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
        .route("/error", get(show_error))
        .with_state(pool)
}
