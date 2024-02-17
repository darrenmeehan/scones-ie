use url::Url;
use scraper::{Html, Selector};

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

pub async fn get_profile_html(profile_uri: Url) -> String {
    let response = reqwest::get(profile_uri).await;
    match response {
        Ok(response) => response.text().await.unwrap(),
        Err(error) => format!("Error: {}", error),
    }
}

pub fn extract_rel_me_links(html: String) -> Vec<url::Url> {
    let fragment = Html::parse_fragment(&html);
    let selector = Selector::parse("a").unwrap();

    let mut result = Vec::new();

    for element in fragment.select(&selector) {
        if let Some(rel) = element.value().attr("rel") {
            let rel_values: Vec<&str> = rel.split(' ').collect();
            if rel_values.contains(&"me") {
                let href = element.value().attr("href").unwrap_or_default();
                let url = Url::parse(href);

                match url {
                    Ok(url) => result.push(url),
                    Err(error) => {
                        tracing::error!("Failed to parse URL: {}. Error: {}", href, error);
                        continue;
                    }
                }
            }
        }
    }
    result
}

pub async fn links_back(to_check: url::Url, check_for: url::Url) -> bool {
    let html = get_profile_html(to_check).await;
    let links = extract_rel_me_links(html);
    links.contains(&check_for)
}
