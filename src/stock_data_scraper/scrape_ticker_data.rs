use crate::fetcher::fetcher_if;
use crate::stock_data_scraper::data_parser_if;
use crate::stock_data_scraper::page_getter::get_page_for_ticker;
use crate::stock_related_types::ticker_data::TickerData;

use log::{error, info};

/**
 * Scrapes each ticker's finviz.com page for TickerData
 * price, p/s, p/b, etc.
 */
pub async fn data_scrape<Parser, Fet>(
    parser: Parser,
    fetcher: Fet,
    tickers: Vec<String>,
    base_page: &str,
) -> impl Iterator<Item = TickerData>
where
    Parser: data_parser_if::DataParserIf,
    Fet: fetcher_if::FetcherIf,
{
    let mut all_ticker_data: Vec<TickerData> = vec![];
    for ticker in &tickers {
        let url = get_page_for_ticker(base_page, ticker);
        match fetcher.fetch_html(&url).await {
            Ok(html) => {
                if let Some(ticker_data) = parser.parse_data(&html, ticker) {
                    info!("Processed page for url: {}", url);
                    all_ticker_data.push(ticker_data);
                }
            }
            Err(e) => {
                error!("Error fetching HTML: {} with url: {}. Stop trying.", e, url);
                break;
            }
        }
    }

    all_ticker_data.into_iter()
}
