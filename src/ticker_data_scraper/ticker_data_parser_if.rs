use crate::ticker_data::ticker_data::TickerData;

pub trait TickerDataParserIf {
    fn extract_data(&self, html: &str, ticker: String) -> TickerData;
}
