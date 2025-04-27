// src/fetcher.rs

use async_trait::async_trait;
use reqwest::Error;

#[async_trait]
pub trait FetcherIf {
    async fn fetch_html(&self, url: &str) -> Result<String, Error>;
}
