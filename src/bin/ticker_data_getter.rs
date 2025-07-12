// src/main.rs

use env_logger;
use finvizor_rust::filtering::pb_filter::PBFilter;
use finvizor_rust::stock_related_types::floating_point::FloatingPoint;
use futures::StreamExt;
use log::error;
use serde::Deserialize;

use finvizor_rust::fetcher::fetcher;
use finvizor_rust::filtering::dividend_filter::DividendFilter;
use finvizor_rust::filtering::filter_if::{NoOpFilter, SecurityFilterIf};
use finvizor_rust::stock_data_scraper::{data_parser, scrape_ticker_data};
use finvizor_rust::ticker_file_reader::ticker_file_reader;

const BASE_PAGE: &str = "https://finviz.com/quote.ashx?t=";
const CONFIG_PATH: &str = "/Users/vowchicke/finvizor_rust/src/configs/ticker_data_getter.toml";

#[derive(Deserialize)]
struct Dividends {
    dividend_ttm: FloatingPoint,
    dividend_est: FloatingPoint,
}

#[derive(Deserialize)]

struct TickerDataGetterOptions {
    dividends: Option<Dividends>,
    pb: Option<FloatingPoint>,
    tickers_path: String,
}

fn decorate_with_dividend_filter_or_return(
    settings: &TickerDataGetterOptions,
    filter: Box<dyn SecurityFilterIf>,
) -> Box<dyn SecurityFilterIf> {
    if let Some(dividends) = &settings.dividends {
        return Box::new(DividendFilter::new(
            Some(filter),
            dividends.dividend_est.clone(),
            dividends.dividend_ttm.clone(),
        ));
    }
    filter
}

fn decorate_with_pb_filter_or_return(
    settings: &TickerDataGetterOptions,
    filter: Box<dyn SecurityFilterIf>,
) -> Box<dyn SecurityFilterIf> {
    if let Some(pb) = &settings.pb {
        return Box::new(PBFilter::new(Some(filter), pb.clone()));
    }
    filter
}

fn decorate_or_return(
    settings: &TickerDataGetterOptions,
    filter: Box<dyn SecurityFilterIf>,
) -> Box<dyn SecurityFilterIf> {
    let filter = decorate_with_dividend_filter_or_return(settings, filter);
    let filter = decorate_with_pb_filter_or_return(settings, filter);
    filter
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let settings: TickerDataGetterOptions = config::Config::builder()
        .add_source(config::File::with_name(CONFIG_PATH))
        .build()?
        .try_deserialize()?;

    let filter = Box::new(NoOpFilter);
    let filter = decorate_or_return(&settings, filter);

    if let Ok(tickers) = ticker_file_reader::read_lines(&settings.tickers_path) {
        let tickers_data = scrape_ticker_data::data_scrape(
            data_parser::DataParser,
            fetcher::Fetcher,
            tickers,
            BASE_PAGE,
        );
        tokio::pin!(tickers_data);

        while let Some(ticker_data) = tickers_data.next().await {
            if filter.filter(&ticker_data) {
                println!(
                    "ticker: {}\ndividend_ttm: {:?}\ndividend_est: {:?}\nprice: {:?}\npb: {:?}",
                    ticker_data.security.finviz_ticker,
                    ticker_data.dividend_ttm,
                    ticker_data.dividend_est,
                    ticker_data.price,
                    ticker_data.pb
                );
                println!();
            }
        }
    } else {
        error!(
            "Error reading tickers fromn file: {}",
            &settings.tickers_path
        );
    }
    Ok(())
}
