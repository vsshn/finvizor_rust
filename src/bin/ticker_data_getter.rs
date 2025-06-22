// src/main.rs

use env_logger;

use finvizor_rust::fetcher::fetcher;
use finvizor_rust::stock_data_scraper::{data_parser, scrape_ticker_data};

const BASE_PAGE: &str = "https://finviz.com/quote.ashx?t=";

#[tokio::main]
async fn main() {
    env_logger::init();

    let tickers = vec!["AAPL".to_string(), "META".to_string()];
    let tickers_data = scrape_ticker_data::data_scrape(
        data_parser::DataParser,
        fetcher::Fetcher,
        tickers,
        BASE_PAGE,
    )
    .await;

    for ticker_data in tickers_data {
        println!("TickerData: {:?}", ticker_data);
    }
}
