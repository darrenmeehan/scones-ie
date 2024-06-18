use crate::github::github_authorize;
use axum::{extract::Query, response::Redirect, Json};
use serde::Serialize;

use std::collections::HashMap;
use url::Url;

pub async fn healthcheck_handler() -> String {
    "All's good".to_string()
}

pub async fn authorization_handler(Query(_params): Query<HashMap<String, String>>) -> Redirect {
    github_authorize().await
}

pub async fn error_handler() -> String {
    let message = "Internal Server Error".to_string();
    format!("Something went wrong: {}", message)
}
