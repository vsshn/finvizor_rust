// src/main.rs

use env_logger;
use finvizor_rust::fetcher::fetcher;
use finvizor_rust::fetcher::fetcher_if::FetcherIf;
use log::error;

#[tokio::main]
async fn main() {
    env_logger::init();

    let fetcher = fetcher::Fetcher {};
    match fetcher
        .fetch_html("https://finviz.com/quote.ashx?t=AACT")
        .await
    {
        Ok(html) => {
            println!("{}", html);
        }
        Err(e) => {
            error!("Error fetching HTML: {}. Stop trying.", e);
        }
    }
}
