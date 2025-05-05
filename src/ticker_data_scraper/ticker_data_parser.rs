use crate::ticker_data::ticker_data;
use crate::ticker_data_scraper::ticker_data_parser_if;

pub struct TickerDataParser;

impl ticker_data_parser_if::TickerDataParserIf for TickerDataParser {
    fn extract_data(&self, _html: &str) -> ticker_data::TickerData {
        let price = ticker_data::Price::new(ticker_data::Value::Positive(5), 1);
        ticker_data::TickerData::new(price, 15.0, 5., "AAPL".to_string())
    }
}
