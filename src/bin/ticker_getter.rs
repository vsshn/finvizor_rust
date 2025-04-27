// src/main.rs

use env_logger;
use finvizor_rust::scraper;
use finvizor_rust::scraper::fetcher;
use finvizor_rust::scraper::next_page_getter;
use finvizor_rust::scraper::ticker_parser;

#[tokio::main]
async fn main() {
    env_logger::init();
    let url = "https://finviz.com/screener.ashx?v=111";
    let tickers = scraper::scrape::scrape(
        next_page_getter::NextPageGetter,
        ticker_parser::TickerParser,
        fetcher::Fetcher,
        url.to_string(),
        10000,
    )
    .await;

    tickers.iter().for_each(|el| {
        println!("{}", el);
    });
}
