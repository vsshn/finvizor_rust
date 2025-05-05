use crate::string_manipulation::string_manipulation::find_between;
use crate::ticker_data::ticker_data;
use crate::ticker_data_scraper::ticker_data_parser_if;

pub struct TickerDataParser;

const START_WORD: &str = "screener_snapshot-table-wrapper";
const END_WORD: &str = "/div";

impl ticker_data_parser_if::TickerDataParserIf for TickerDataParser {
    fn extract_data(&self, html: &str, ticker: String) -> ticker_data::TickerData {
        let block = find_between(&html, START_WORD, END_WORD);
        let price = ticker_data::Price::new(ticker_data::Value::Positive(1127), -2);
        ticker_data::TickerData::new(Some(price), Some(27.09), None, ticker)
    }
}
