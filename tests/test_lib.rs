use scones::{extract_auth_endpoint, extract_rel_me_links};

#[cfg(test)]
mod tests {
    use url::Url;

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

    #[tokio::test]
    async fn extract_rel_me_links_one_link() {
        let html = r#"
        <html>
        <a rel="me" href="https://scones.fly.dev/auth">
            <body>
            <h1>Test</h1>
            </body>
        </html>
        "#
        .to_string();

        let actual_result = extract_rel_me_links(html);
        let mut expected_result = Vec::new();
        expected_result.push(Url::parse("https://scones.fly.dev/auth").unwrap());
        assert_eq!(expected_result, actual_result)
    }

    #[tokio::test]
    async fn extract_rel_me_links_two_links() {
        let html = r#"
        <html>
        <a rel="me" href="https://scones.fly.dev/1">
        <a rel="nofollow me" href="https://scones.fly.dev/2">
            <body>
            <h1>Test</h1>
            </body>
        </html>
        "#
        .to_string();

        let actual_result = extract_rel_me_links(html);
        let mut expected_result = Vec::new();
        expected_result.push(Url::parse("https://scones.fly.dev/1").unwrap());
        expected_result.push(Url::parse("https://scones.fly.dev/2").unwrap());
        assert_eq!(expected_result, actual_result)
    }
    #[tokio::test]
    async fn extract_rel_me_links_no_links() {
        let html = r#"
        <html>
        <a rel="you" href="https://scones.fly.dev/">
            <body>
            <h1>Test</h1>
            </body>
        </html>
        "#
        .to_string();

        let actual_result: Vec<Url> = extract_rel_me_links(html);
        let expected_result: Vec<Url> = Vec::new();
        assert_eq!(expected_result, actual_result)
    }
}
