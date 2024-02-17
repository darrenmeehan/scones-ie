use scones::app;
use scones::database::connect;

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn blackbox() {
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, app(connect().await)).await.unwrap();
        });

        let client = reqwest::Client::new();

        let response = client
            .get(format!("http://{addr}/healthcheck"))
            .send()
            .await
            .expect("Failed to execute request");

        assert!(response.status().is_success());
        assert_eq!(Some(10), response.content_length());
    }
}
