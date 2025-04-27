// src/parser.rs

use crate::ticker_scraper::ticker_parser_if;

fn find_between<'a>(text: &'a str, start_word: &str, end_word: &str) -> Option<&'a str> {
    // Find the position of the start word
    let start_pos = text.find(start_word)? + start_word.len();
    // Find the position of the end word after the start word
    let end_pos = text[start_pos..].find(end_word)? + start_pos;
    // Return the substring between them
    Some(&text[start_pos..end_pos])
}

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
