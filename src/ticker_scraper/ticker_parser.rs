// src/parser.rs

use crate::string_manipulation::string_manipulation::find_between;
use crate::ticker_scraper::ticker_parser_if;

fn extract_tickers_from_block_non_opt(block: &str) -> Vec<String> {
    block
        .split('\n')
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if !parts.is_empty() {
                let ticker = parts[0].to_string();
                if ticker.is_empty() {
                    None
                } else {
                    Some(ticker)
                }
            } else {
                None
            }
        })
        .collect()
}

fn extract_tickers_from_block(block: Option<&str>) -> Vec<String> {
    match block {
        Some(values) => extract_tickers_from_block_non_opt(&values),
        None => vec![],
    }
}

const START_WORD: &str = "!-- TS";
const END_WORD: &str = "TE -->";

pub struct TickerParser;

impl ticker_parser_if::TickerParserIf for TickerParser {
    fn extract_tickers(&self, html: &str) -> Vec<String> {
        let block = find_between(html, START_WORD, END_WORD);
        let tickers = extract_tickers_from_block(block);
        tickers
    }
}
