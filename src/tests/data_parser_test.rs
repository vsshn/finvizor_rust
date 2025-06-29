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
        Some(TickerData {
            security: Security {
                finviz_ticker: "META".to_string()
            },
            price: Some(floating_point::FloatingPoint::new(1127, -2)),
            pb: Some(FloatingPoint::new(133, -2)),
            dividend_ttm: Some(FloatingPoint::new(50, -2)),
            dividend_est: Some(FloatingPoint::new(51, -2)),
            pe: Some(FloatingPoint::new(2709, -2))
        })
    )
}

#[test]
fn test_parse_data_empty() {
    let html = "".to_string();
    assert_eq!(DataParser.parse_data(&html, "META"), None);
}
