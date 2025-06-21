/// ```
/// use finvizor_rust::stock_data_scraper::page_getter::get_page_for_ticker;
/// const base_page: &str = "https://finviz.com/quote.ashx?t=";
/// assert_eq!(get_page_for_ticker(base_page, "META"), "https://finviz.com/quote.ashx?t=META");
/// assert_eq!(get_page_for_ticker(base_page, "AAPL"), "https://finviz.com/quote.ashx?t=AAPL");
/// ```
pub fn get_page_for_ticker(base_page: &str, ticker: &str) -> String {
    format!("{}{}", base_page, ticker)
}
