use crate::scraper::fetcher_if;
use async_trait::async_trait;
use reqwest::Client;
use reqwest::Error;

pub struct Fetcher;

#[async_trait]
impl fetcher_if::FetcherIf for Fetcher {
    async fn fetch_html(&self, url: &str) -> Result<String, Error> {
        let client = Client::builder().build()?;
        let response = client
            .get(url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await?;
        let body = response.text().await?;
        Ok(body)
    }
}
