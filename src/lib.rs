use axum::{response::Html, routing::get, Json, Router};
use serde::Serialize;
use tower_http::services::ServeFile;

pub async fn run() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}

async fn scone_handler() -> Html<&'static str> {
    Html("<h1>They're called scONes!</h1>")
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
        issuer: "".to_string(),
        authorization_endpoint: "".to_string(),
        token_endpoint: "".to_string(),
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
        .route("/", get(scone_handler))
        .route("/healthcheck", get(healthcheck_handler))
        .route_service("/private", ServeFile::new("static/index.html"))
        .route("/metadata", get(metadata_handler))
        .route("/authorization", get(authorization_handler))
        .route("/token", get(token_handler))
}
