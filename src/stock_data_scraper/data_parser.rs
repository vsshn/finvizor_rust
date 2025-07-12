use crate::stock_data_scraper::data_parser_if;
use crate::stock_related_types::{
    floating_point::FloatingPoint, security::Security, ticker_data::TickerData,
};
use crate::string_manipulation::string_manipulation::find_between;

use log::error;
use once_cell::sync::Lazy;
use scraper::{Html, Selector};
use std::collections::HashSet;

static FUNDAMENTAL_PARAMETERS: Lazy<HashSet<String>> = Lazy::new(|| {
    [
        "P/B".to_string(),
        "Price".to_string(),
        "Dividend TTM".to_string(),
        "Dividend Est.".to_string(),
        "P/E".to_string(),
        "P/S".to_string(),
        "P/C".to_string(),
    ]
    .iter()
    .cloned()
    .collect()
});

pub struct DataParser;

fn get_all_tds_from_page(document: &Html) -> Option<Vec<scraper::ElementRef>> {
    let div_selector = Selector::parse("div.screener_snapshot-table-wrapper").unwrap();
    let table_selector = Selector::parse("table").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    if let Some(div) = document.select(&div_selector).next() {
        // Find the table inside the div
        if let Some(table) = div.select(&table_selector).next() {
            // Iterate over all <td> elements
            return Some(table.select(&td_selector).collect());
        }
    }

    return None;
}

fn get_string_from_element_ref(element_ref: &scraper::ElementRef) -> String {
    element_ref
        .text()
        .collect::<Vec<_>>()
        .join("")
        .trim()
        .to_string()
}

fn extract_percent(value: &str) -> Option<FloatingPoint> {
    if let Some(percent) = find_between(value, "(", ")") {
        let percent = &percent[..percent.len().saturating_sub(1)];
        FloatingPoint::construct_from_string(percent)
    } else {
        None
    }
}

fn set_value_for_element(ticker_data: &mut TickerData, element: &str, value: &String) {
    match element {
        "P/B" => ticker_data.pb = FloatingPoint::construct_from_string(&value),
        "Price" => ticker_data.price = FloatingPoint::construct_from_string(&value),
        "Dividend TTM" => ticker_data.dividend_ttm = extract_percent(value),
        "Dividend Est." => ticker_data.dividend_est = extract_percent(value),
        "P/E" => ticker_data.pe = FloatingPoint::construct_from_string(value),
        "P/S" => ticker_data.ps = FloatingPoint::construct_from_string(value),
        "P/C" => ticker_data.pc = FloatingPoint::construct_from_string(value),
        _ => println!("smth else: {} : {}", element, value),
    }
}

fn process_tds(tds: Vec<scraper::ElementRef>) -> TickerData {
    let mut left_to_find = FUNDAMENTAL_PARAMETERS.clone();
    let mut ticker_data = TickerData::default();
    for chunk in tds.chunks(2) {
        if chunk.len() != 2 {
            error!("Uneven number of elements in a table. Exiting");
            break;
        }

        let element = get_string_from_element_ref(&chunk[0]);
        if left_to_find.remove(&element) {
            let value = get_string_from_element_ref(&chunk[1]);
            set_value_for_element(&mut ticker_data, &element, &value);
        }
    }
    ticker_data
}

impl data_parser_if::DataParserIf for DataParser {
    fn parse_data(&self, html: &str, ticker: &str) -> Option<TickerData> {
        let document = Html::parse_document(&html);

        if let Some(tds) = get_all_tds_from_page(&document) {
            let mut ticker_data = process_tds(tds);
            ticker_data.security = Security {
                finviz_ticker: ticker.to_string(),
            };
            return Some(ticker_data);
        }

        None
    }
}
