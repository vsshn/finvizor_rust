use crate::stock_related_types::ticker_data::TickerData;

pub trait DataParserIf {
    fn parse_data(&self, html: &str, ticker: &str) -> Option<TickerData>;
}
