// src/main.rs

use env_logger;
use finvizor_rust::fetcher::fetcher;
use finvizor_rust::next_page_getter::next_page_getter;
use finvizor_rust::ticker_scraper;
use finvizor_rust::ticker_scraper::ticker_parser;

#[tokio::main]
async fn main() {
    env_logger::init();
    let url = "https://finviz.com/screener.ashx?v=111";
    let tickers = ticker_scraper::scrape::scrape(
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
