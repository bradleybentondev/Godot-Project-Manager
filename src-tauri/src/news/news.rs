use regex::Regex;
use reqwest::header::USER_AGENT;
use scraper::{selectable::Selectable, ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

use crate::environmnet::is_prod;

const NEWS_URL: &str = "https://godotengine.org/blog/";

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsEntry {
    title: String,
    info: String,
    body: String,
    image_url: String,
    href: String,
}

pub async fn get_news() -> Option<Vec<NewsEntry>> {
    let html_string = download_news().await?;

    Some(parse_html_string(&html_string).await)
}

async fn download_news() -> Option<String> {
    let client = reqwest::Client::new();
    let body: String = client
        .get(NEWS_URL)
        .header(USER_AGENT, "My Rust Program 1.0")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Some(body)
}

async fn parse_html_string(html: &str) -> Vec<NewsEntry> {
    let document = Html::parse_document(html);

    let selector = Selector::parse("article").unwrap();

    document
        .select(&selector)
        .into_iter()
        .map(|element| build_news_entry(element))
        .collect::<Vec<NewsEntry>>()
}

fn build_news_entry(element: ElementRef) -> NewsEntry {
    let title_selector = Selector::parse("h3").unwrap();
    let excerpt_selector = Selector::parse("p").unwrap();
    let by_selector = Selector::parse(".by").unwrap();
    let date_selector = Selector::parse(".date").unwrap();
    let thumbnail_selector = Selector::parse(".thumbnail").unwrap();

    let re = Regex::new(r"\(([^)]+)\)").unwrap();

    let mut info = element
        .select(&by_selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap()
        .to_string();

    info = info
        + element
            .select(&date_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();

    let image_url = element.select(&thumbnail_selector).next().unwrap();
    let image_url_html = image_url.html();
    let href = image_url.value().attr("href").unwrap();
    let mut image_url = "".to_string();

    if let Some(captured) = re.captures(&image_url_html) {
        image_url = "https://godotengine.org".to_string() + &captured[1];
    }

    NewsEntry {
        title: element
            .select(&title_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .to_string(),
        body: element
            .select(&excerpt_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .to_string(),
        info: info,
        image_url,
        href: href.to_string(),
    }
}

mod tests {
    use super::parse_html_string;
    use crate::news::news::download_news;

    #[tokio::test]
    async fn test_fetching_news_page() {
        let body = download_news().await.unwrap();

        println!("body: {}", body);

        assert!(body.len() > 0)
    }

    #[tokio::test]
    async fn test_parsing_body() {
        let body = download_news().await.unwrap();
        let parsed = parse_html_string(&body).await;

        println!("parsed: {:#?}", &parsed);
        assert!(parsed.len() > 0);
    }
}
