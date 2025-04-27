pub trait TickerParserIf {
    fn extract_tickers(&self, html: &str) -> Vec<String>;
}
