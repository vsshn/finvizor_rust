use log::{debug, error, info};

use crate::fetcher::fetcher_if;
use crate::ticker_data::ticker_data::TickerData;
use crate::ticker_data_scraper::ticker_data_parser_if;
use crate::ticker_scraper::next_page_getter_if;

pub async fn scrape<NPG, Parser, Fet>(
    npg: NPG,
    parser: Parser,
    fetch: Fet,
    base_page: String,
    max_pages: u32,
) -> impl Iterator<Item = TickerData>
where
    NPG: next_page_getter_if::NextPageGetterIf,
    Parser: ticker_data_parser_if::TickerDataParserIf,
    Fet: fetcher_if::FetcherIf,
{
    let mut all_ticker_data: Vec<TickerData> = vec![];
    for page in 0..max_pages {
        let current_page = match npg.get_next_page(&base_page, page) {
            Some(page) => page,
            None => {
                info!("Last ticker done");
                return all_ticker_data.into_iter();
            }
        };

        debug!("current page: {}", current_page);
        match fetch.fetch_html(&current_page).await {
            Ok(html) => {
                let ticker_data = parser.extract_data(&html);
                all_ticker_data.push(ticker_data);
            }
            Err(e) => {
                error!("Error fetching HTML: {}. Stop trying.", e);
                break;
            }
        }
    }
    all_ticker_data.into_iter()
}
