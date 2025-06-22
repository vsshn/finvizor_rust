use crate::fetcher::fetcher_if;
use crate::stock_data_scraper::data_parser_if;
use crate::stock_data_scraper::page_getter::get_page_for_ticker;
use crate::stock_related_types::ticker_data::TickerData;

use log::{error, info};

use async_stream::stream;
use futures_core::stream::Stream;
/**
 * Scrapes each ticker's finviz.com page for TickerData
 * price, p/s, p/b, etc.
 */
pub fn data_scrape<'a, Parser, Fet>(
    parser: Parser,
    fetcher: Fet,
    tickers: Vec<String>,
    base_page: &'a str,
) -> impl Stream<Item = TickerData> + 'a
where
    Parser: data_parser_if::DataParserIf + 'a,
    Fet: fetcher_if::FetcherIf + 'a,
{
    stream! {
        for ticker in tickers {
            let url = get_page_for_ticker(base_page, &ticker);
            match fetcher.fetch_html(&url).await {
                Ok(html) => {
                    if let Some(ticker_data) = parser.parse_data(&html, &ticker) {
                        info!("Processed page for url: {}", url);
                        yield ticker_data;
                    }
                }
                Err(e) => {
                    error!("Error fetching HTML: {} with url: {}. Stop trying.", e, url);
                    break;
                }
            }
        }
    }
}
