use crate::ticker_scraper::next_page_getter_if::NextPageGetterIf;

pub struct NextPageGetter {
    tickers: Vec<String>,
}

impl NextPageGetter {
    pub fn new(tickers: Vec<String>) -> Self {
        Self { tickers }
    }
}

impl NextPageGetterIf for NextPageGetter {
    fn get_next_page(&self, base_page: &str, page_num: u32) -> Option<String> {
        for el in &self.tickers {
            println!("{}", el);
        }
        // "https://finviz.com/quote.ashx?t=AACT&ty=c&p=d&b=1"
        if let Some(ticker) = self.tickers.get(page_num as usize) {
            Some(format!("{}{}", base_page, ticker))
        } else {
            None
        }
    }
}
