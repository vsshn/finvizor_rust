use crate::ticker_scraper::next_page_getter_if;

pub struct NextPageGetter;

const TICKERS_PER_PAGE: u32 = 20;

impl next_page_getter_if::NextPageGetterIf for NextPageGetter {
    fn get_next_page(&self, base_page: &str, page_num: u32) -> Option<String> {
        //https://finviz.com/screener.ashx?v=111&r=21
        Some(format!(
            "{}&r={}",
            base_page,
            TICKERS_PER_PAGE * page_num + 1
        ))
    }
}
