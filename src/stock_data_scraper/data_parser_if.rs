use crate::stock_related_types::ticker_data::TickerData;
use mockall::automock;

#[automock]
pub trait DataParserIf {
    fn parse_data(&self, html: &str, ticker: &str) -> Option<TickerData>;
}
