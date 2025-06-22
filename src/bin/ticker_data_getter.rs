// src/main.rs

use env_logger;
use futures::StreamExt;
use log::error;

use finvizor_rust::fetcher::fetcher;
use finvizor_rust::stock_data_scraper::{data_parser, scrape_ticker_data};
use finvizor_rust::ticker_file_reader::ticker_file_reader;

const BASE_PAGE: &str = "https://finviz.com/quote.ashx?t=";
const TICKERS_FILE_PATH: &str = "/Users/vowchicke/finvizor_rust/tickers.txt";

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Ok(tickers) = ticker_file_reader::read_lines(TICKERS_FILE_PATH) {
        let tickers_data = scrape_ticker_data::data_scrape(
            data_parser::DataParser,
            fetcher::Fetcher,
            tickers,
            BASE_PAGE,
        );
        tokio::pin!(tickers_data);

        while let Some(ticker_data) = tickers_data.next().await {
            println!("TickerData: {:?}", ticker_data);
        }
    } else {
        error!("Error reading tickers fromn file: {}", TICKERS_FILE_PATH);
    }
}
