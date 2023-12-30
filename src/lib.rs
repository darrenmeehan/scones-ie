use axum::{routing::get, Json, Router};
use serde::Serialize;
use tower_http::services::ServeFile;

pub async fn run() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}

async fn healthcheck_handler() -> String {
    "All's good".to_string()
}

async fn authorization_handler() -> String {
    "Auth endpoint not implemented".to_string()
}

async fn token_handler() -> String {
    "Token endpoint not implemented".to_string()
}

async fn metadata_handler() -> Json<MetaData> {
    let metadata = MetaData {
        issuer: "https://scones.fly.dev".to_string(),
        authorization_endpoint: "https://scones.fly.dev/auth".to_string(),
        token_endpoint: "https://scones.fly.dev/token".to_string(),
        code_challenge_methods_supported: "".to_string(),
    };
    Json(metadata)
}

#[derive(Serialize)]
struct MetaData {
    issuer: String,
    authorization_endpoint: String,
    token_endpoint: String,
    // list of supported methods
    code_challenge_methods_supported: String,
}

pub fn app() -> Router {
    Router::new()
        .route("/healthcheck", get(healthcheck_handler))
        .route_service("/", ServeFile::new("static/index.html"))
        .route_service("/logo.jpg", ServeFile::new("static/logo.jpg"))
        .route_service("/login", ServeFile::new("static/login.html"))
        .route_service("/darren", ServeFile::new("static/profile.html"))
        .route("/.well-known/oauth-authorization-server", get(metadata_handler))
        .route("/metadata", get(metadata_handler))
        .route("/auth", get(authorization_handler))
        .route("/token", get(token_handler))
}
