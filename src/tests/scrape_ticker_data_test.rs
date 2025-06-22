use crate::fetcher::fetcher_if::MockFetcherIf;
use crate::stock_data_scraper::data_parser_if::MockDataParserIf;
use crate::stock_data_scraper::scrape_ticker_data;
use crate::stock_related_types::{
    floating_point::FloatingPoint, security::Security, ticker_data::TickerData,
};

use mockall::predicate::*;

const BASE_PAGE: &str = "https://finviz.com/quote.ashx?t=";

#[tokio::test]
async fn test_scrape_ticker_data_empty() {
    let tickers = vec![];
    let mut parser = MockDataParserIf::new();
    let mut fetcher = MockFetcherIf::new();
    parser.expect_parse_data().never();
    fetcher.expect_fetch_html().never();
    let _tickers_data = scrape_ticker_data::data_scrape(parser, fetcher, tickers, BASE_PAGE).await;
}

#[tokio::test]
async fn test_scrape_ticker_data_normal() {
    let meta: String = "META".to_string();
    let aapl: String = "AAPL".to_string();
    let html = "HTML".to_string();

    let meta_price = FloatingPoint::new(133, 1);
    let apple_price = FloatingPoint::new(123, -2);
    let meta_pb = FloatingPoint::new(1234, -1);
    let apple_pb = FloatingPoint::new(1, 1);

    let tickers = vec![meta.clone(), aapl.clone()];
    let mut parser = MockDataParserIf::new();
    let mut fetcher = MockFetcherIf::new();
    fetcher
        .expect_fetch_html()
        .with(eq("https://finviz.com/quote.ashx?t=META".to_string()))
        .once()
        .return_once({
            let html = html.clone();
            |_| Ok(html)
        });
    fetcher
        .expect_fetch_html()
        .with(eq("https://finviz.com/quote.ashx?t=AAPL".to_string()))
        .once()
        .return_once({
            let html = html.clone();
            |_| Ok(html)
        });

    parser
        .expect_parse_data()
        .with(eq(html.clone()), eq(meta.clone()))
        .once()
        .return_once({
            let meta = meta.clone();
            let meta_price = meta_price.clone();
            let meta_pb = meta_pb.clone();
            |_, _| {
                Some(TickerData::new(
                    Security {
                        finviz_ticker: meta,
                    },
                    Some(meta_price),
                    Some(meta_pb),
                ))
            }
        });
    parser
        .expect_parse_data()
        .with(eq(html.clone()), eq(aapl.clone()))
        .once()
        .return_once({
            let aapl = aapl.clone();
            let apple_price = apple_price.clone();
            let apple_pb = apple_pb.clone();
            |_, _| {
                Some(TickerData::new(
                    Security {
                        finviz_ticker: aapl,
                    },
                    Some(apple_price),
                    Some(apple_pb),
                ))
            }
        });
    let tickers_data = scrape_ticker_data::data_scrape(parser, fetcher, tickers, BASE_PAGE).await;
    let vec_tickers_data: Vec<TickerData> = tickers_data.collect();
    assert_eq!(
        vec_tickers_data,
        vec![
            TickerData::new(
                Security {
                    finviz_ticker: meta.clone(),
                },
                Some(meta_price.clone()),
                Some(meta_pb.clone()),
            ),
            TickerData::new(
                Security {
                    finviz_ticker: aapl.clone(),
                },
                Some(apple_price.clone()),
                Some(apple_pb.clone()),
            )
        ]
    )
}
