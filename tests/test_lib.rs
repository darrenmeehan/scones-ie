use scones::extract_auth_endpoint;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn extract_auth_endpoint_works() {
        let html = r#"
        <html>
            <head>
                <link rel="authorization_endpoint" href="https://scones.fly.dev/auth">
            </head>
            <body>
            <h1>Test</h1>
            </body>
        </html>
        "#
        .to_string();
        let actual_result = extract_auth_endpoint(html);
        let expected_result = "https://scones.fly.dev/auth".to_string();
        assert_eq!(expected_result, actual_result)
    }
}
