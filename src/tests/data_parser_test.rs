use crate::{
    stock_data_scraper::{data_parser::DataParser, data_parser_if::DataParserIf},
    stock_related_types::{
        floating_point::{self, FloatingPoint},
        security::Security,
        ticker_data::TickerData,
    },
};

use std::fs;

const HTML_PATH: &str = "/Users/vowchicke/finvizor_rust/src/tests/html_one_page.html";

#[test]
fn test_parse_data_normal() {
    let html = fs::read_to_string(HTML_PATH).unwrap();
    assert_eq!(
        DataParser.parse_data(&html, "META"),
        TickerData::new(
            Security {
                finviz_ticker: "META".to_string()
            },
            Some(floating_point::FloatingPoint::new(1127, -2)),
            Some(FloatingPoint::new(133, -1)),
        )
    )
}
