// src/fetcher.rs

use async_trait::async_trait;
use mockall::automock;
use reqwest::Error;

#[automock]
#[async_trait]
pub trait FetcherIf {
    async fn fetch_html(&self, url: &str) -> Result<String, Error>;
}
