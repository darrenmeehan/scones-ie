use serde::Serialize;
use crate::github::github_authorize;
use crate::relme::{links_back, get_profile_html, extract_rel_me_links};
use axum::{extract::Query, response::Redirect, Json};

use url::Url;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct MetaData {
    issuer: String,
    authorization_endpoint: String,
    token_endpoint: String,
    // list of supported methods
    code_challenge_methods_supported: String,
}

pub async fn healthcheck_handler() -> String {
    "All's good".to_string()
}

pub async fn authorization_handler(Query(_params): Query<HashMap<String, String>>) -> Redirect {
    github_authorize().await
}

pub async fn token_handler() -> String {
    "Token endpoint not implemented".to_string()
}

pub async fn metadata_handler() -> Json<MetaData> {
    let metadata = MetaData {
        issuer: "https://scones.fly.dev".to_string(),
        authorization_endpoint: "https://scones.fly.dev/auth".to_string(),
        token_endpoint: "https://scones.fly.dev/token".to_string(),
        code_challenge_methods_supported: "".to_string(),
    };
    Json(metadata)
}

pub async fn client_handler(Query(params): Query<HashMap<String, String>>) -> Redirect {
    let profile_uri = Url::parse(params.get("me").unwrap()).unwrap();
    let html = get_profile_html(profile_uri.clone()).await;

    let links = extract_rel_me_links(html);

    for link in links {
        if links_back(link.clone(), profile_uri.clone()).await
            && link.domain() == Some("github.com")
        {
            return github_authorize().await;
        }
    }
    Redirect::temporary("/error")
}

pub async fn error_handler() -> String {
    let message = "Could not find suitable RelMeAuth endpoint".to_string();
    format!("Something went wrong: {}", message)
}
