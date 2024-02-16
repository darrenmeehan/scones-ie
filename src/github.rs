pub(crate) use std::{collections::HashMap, env, result::Result};

use axum::{extract::Query, response::Redirect};
use reqwest::header;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GithubUser {
    pub id: u64,
    pub blog: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GithubCredentials {
    /// example: "90sd098w0e98f0w9e8g90a8ed098"
    pub access_token: String,
    /// example: "read:user"
    pub scope: String,
    /// example: "bearer"
    pub token_type: String,
}

pub async fn github_authorize() -> Redirect {
    let client_id = env::var("GITHUB_CLIENT_ID").expect("Expected CLIENT_ID to be set.");
    let uri = format!(
        "https://github.com/login/oauth/authorize?client_id={}",
        client_id
    );
    Redirect::temporary(&uri)
}

pub async fn callback_handler(Query(params): Query<HashMap<String, String>>) -> Redirect {
    let code = params.get("code").unwrap().to_string();
    let credentials = get_user_credentials(code.clone()).await;

    match credentials {
        Ok(credentials) => {
            let user = get_user(credentials).await;
            match user.unwrap().blog {
                blog if blog == "https://drn.ie" => {
                    let message = "Authorizing";
                    Redirect::temporary(&format!("/success?message={}&code={}", message, &code))
                }
                _ => Redirect::temporary("/error?reason=not-admin"),
            }
        }
        Err(_) => Redirect::temporary("/error?reason=github-error"),
    }
}

pub async fn get_user_credentials(code: String) -> Result<GithubCredentials, Error> {
    let client_id = env::var("GITHUB_CLIENT_ID").expect("Expected CLIENT_ID to be set.");
    let client_secret = env::var("GITHUB_CLIENT_SECRET").expect("Missing CLIENT_SECRET!");

    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("code", code),
    ];

    let client = reqwest::Client::new();
    client
        .post("https://github.com/login/oauth/access_token")
        .timeout(std::time::Duration::from_secs(3))
        .header(header::ACCEPT, "application/json")
        .form(&params)
        .send()
        .await
        .unwrap()
        .json::<GithubCredentials>()
        .await
}

async fn get_user(credentials: GithubCredentials) -> Result<GithubUser, Error> {
    let client = reqwest::Client::new();
    client
        .get("https://api.github.com/user")
        .timeout(std::time::Duration::from_secs(3))
        .header(header::ACCEPT, "application/json")
        .header(header::CONTENT_TYPE, "application/json")
        .header(
            header::AUTHORIZATION,
            format!("bearer {}", credentials.access_token),
        )
        .header(header::USER_AGENT, "github.com/darrenmeehan/scones-ie")
        .send()
        .await
        .unwrap()
        .json::<GithubUser>()
        .await
}
