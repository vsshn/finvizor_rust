use std::collections::HashSet;

use crate::fetcher::fetcher_if;
use crate::ticker_scraper::next_page_getter_if;
use crate::ticker_scraper::ticker_parser_if;

use log::{debug, error, info};

pub async fn scrape<NPG, T, Fet>(
    npg: NPG,
    parser: T,
    fetch: Fet,
    base_page: String,
    max_pages: u32,
) -> HashSet<String>
where
    NPG: next_page_getter_if::NextPageGetterIf,
    T: ticker_parser_if::TickerParserIf,
    Fet: fetcher_if::FetcherIf,
{
    let mut current_page = base_page.clone();
    let mut all_tickers: HashSet<String> = HashSet::new();
    for page in 1..max_pages {
        debug!("current page: {}", current_page);
        match fetch.fetch_html(&current_page).await {
            Ok(html) => {
                let tickers = parser.extract_tickers(&html);
                for ticker in tickers {
                    if !all_tickers.insert(ticker) {
                        info!("Found duplicate ticker. Stopping scrapping...");
                        return all_tickers;
                    }
                }
            }
            Err(e) => {
                error!("Error fetching HTML: {}. Stop trying.", e);
                break;
            }
        }
        current_page = npg.get_next_page(&base_page, page).unwrap();
    }
    all_tickers
}
