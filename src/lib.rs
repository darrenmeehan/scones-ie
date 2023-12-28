use axum::{response::Html, routing::get, Router};
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

pub fn app() -> Router {
    Router::new()
        .route("/", get(scone_handler))
        .route("/healthcheck", get(healthcheck_handler))
        .route_service("/private", ServeFile::new("static/index.html"))
}
