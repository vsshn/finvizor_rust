// src/main.rs

use env_logger;
use finvizor_rust::fetcher::fetcher;
use finvizor_rust::ticker_data_scraper::data_scrape::scrape;
use finvizor_rust::ticker_data_scraper::next_page_getter;
use finvizor_rust::ticker_data_scraper::ticker_data_parser;

#[tokio::main]
async fn main() {
    env_logger::init();
    let url = "https://finviz.com/quote.ashx?t=";
    let tickers_data = scrape(
        next_page_getter::NextPageGetter::new(vec!["AAPL".to_string()]),
        ticker_data_parser::TickerDataParser,
        fetcher::Fetcher,
        url.to_string(),
        10000,
    )
    .await;

    for element in tickers_data {
        element.print();
    }
}
