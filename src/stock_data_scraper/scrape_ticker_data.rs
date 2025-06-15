use std::collections::HashSet;

use crate::fetcher::fetcher_if;
use crate::next_page_getter::next_page_getter_if;
use crate::stock_data_scraper::data_parser_if;

use log::{debug, error, info};

/**
 * Scrapes each ticker's finviz.com page for TickerData
 * price, p/s, p/b, etc.
 */
pub async fn data_scrape<NPG, Parser, Fet>(
    npg: NPG,
    parser: Parser,
    fetch: Fet,
    base_page: String,
    max_pages: u32,
) -> HashSet<String>
where
    NPG: next_page_getter_if::NextPageGetterIf,
    Parser: data_parser_if::DataParserIf,
    Fet: fetcher_if::FetcherIf,
{
}
