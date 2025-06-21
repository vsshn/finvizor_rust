use crate::stock_data_scraper::page_getter_if;

pub struct PageGetter {
    tickers: Vec<String>,
    base_page: String,
    current_index: usize,
}

impl PageGetter {
    pub fn new(tickers: Vec<String>, base_page: String) -> Self {
        Self {
            tickers,
            base_page,
            current_index: 0,
        }
    }
}

// base_page = "https://finviz.com/quote.ashx?t=";

fn get_page_for_ticker(ticker: &str, base_page: &str) -> String {
    format!("{}{}", base_page, ticker)
}

impl page_getter_if::PageGetterIf for PageGetter {
    fn get_next_page(&mut self) -> Option<String> {
        match self.current_index {
            x if x < self.tickers.len() => {
                self.current_index = x + 1;
                Some(get_page_for_ticker(&self.tickers[x], &self.base_page))
            }
            _ => None,
        }
    }
}
