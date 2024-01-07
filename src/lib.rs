use std::collections::HashMap;

use axum::{extract::Query, response::Redirect, routing::get, Json, Router};
use reqwest;
use scraper::{Html, Selector};
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

async fn authorization_handler(Query(params): Query<HashMap<String, String>>) -> String {
    
    let client_html = get_profile_html(params.get("client_uri").unwrap().to_string()).await;
    // Get client info
    // Get user info
    // auth user by email
    client_html
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

pub async fn client_handler(Query(params): Query<HashMap<String, String>>) -> Redirect {
    let profile_uri = params.get("me").unwrap().to_string();
    let profile_uri_clone = profile_uri.clone();
    let html = get_profile_html(profile_uri_clone).await;

    let auth_endpoint = extract_auth_endpoint(html);

    let redirection_uri = build_auth_request(auth_endpoint, profile_uri);
    Redirect::permanent(&redirection_uri)
}

pub fn extract_auth_endpoint(html: String) -> String {
    let fragment = Html::parse_fragment(&html);
    let selector = Selector::parse("link").unwrap();

    let mut result = String::new();

    for element in fragment.select(&selector) {
        if let Some(rel) = element.value().attr("rel") {
            if rel == "authorization_endpoint" {
                let auth_endpoint = element.value().attr("href").unwrap_or_default();
                result.push_str(auth_endpoint);
            }
        }
    }
    result
}

async fn get_profile_html(profile_uri: String) -> String {
    let response = reqwest::get(profile_uri).await;
    let html = match response {
        Ok(response) => response.text().await.unwrap(),
        Err(error) => format!("Error: {}", error),
    };
    html
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
        .route_service("/darren.jpg", ServeFile::new("static/darren.jpg"))
        .route_service("/login", ServeFile::new("static/login.html"))
        .route_service("/darren", ServeFile::new("static/profile.html"))
        .route(
            "/.well-known/oauth-authorization-server",
            get(metadata_handler),
        )
        .route("/metadata", get(metadata_handler))
        .route("/auth", get(authorization_handler))
        .route("/token", get(token_handler))
        .route("/client", get(client_handler))
}
