use crate::ticker_scraper::{ticker_parser, ticker_parser_if::TickerParserIf};

#[test]
fn test_ticker_parser_normal_one() {
    assert_eq!(
        ticker_parser::TickerParser.extract_tickers("!-- TS\nABC| TE -->"),
        vec!["ABC"],
    );
}

#[test]
fn test_ticker_parser_normal_many() {
    assert_eq!(
        ticker_parser::TickerParser.extract_tickers(
            "!-- TS\nABC|dsjafkjhdfaj|fdfad\nAAPL|dfjkjdkh\nMSFT|fds|afads||s TE -->"
        ),
        vec!["ABC", "AAPL", "MSFT"],
    );
}

#[test]
fn test_ticker_parser_no_tickers() {
    let empty_vec: Vec<String> = Vec::new();
    assert_eq!(
        ticker_parser::TickerParser.extract_tickers(
            "!-- TS\nABC|dsjafkjhdfaj|fdfad\nAAPL|dfjkjdkh\nMSFT|fds|afads||s TZ -->"
        ),
        empty_vec,
    );
}

#[test]
fn test_ticker_parser_empty_html() {
    let empty_vec: Vec<String> = Vec::new();
    assert_eq!(ticker_parser::TickerParser.extract_tickers(""), empty_vec,);
}
