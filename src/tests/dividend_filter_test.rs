use crate::filtering::dividend_filter::DividendFilter;
use crate::filtering::filter_if::{NoOpFilter, SecurityFilterIf};
use crate::stock_related_types::floating_point::FloatingPoint;
use crate::stock_related_types::security::Security;
use crate::stock_related_types::ticker_data::TickerData;

fn create_ticker_data(
    div_est: Option<FloatingPoint>,
    div_ttm: Option<FloatingPoint>,
) -> TickerData {
    TickerData {
        dividend_ttm: div_ttm,
        dividend_est: div_est,
        security: Security {
            finviz_ticker: "".to_string(),
        },
        price: None,
        pb: None,
        pe: None,
    }
}

#[test]
fn test_filter_pass() {
    let div_filter: DividendFilter<NoOpFilter> =
        DividendFilter::new(None, FloatingPoint::new(1, 0), FloatingPoint::new(1, 1));
    assert!(div_filter.filter(&create_ticker_data(
        Some(FloatingPoint::new(1, 2)),
        Some(FloatingPoint::new(2, 2))
    )));

    let div_filter: DividendFilter<NoOpFilter> =
        DividendFilter::new(None, FloatingPoint::new(1, 2), FloatingPoint::new(2, 1));
    assert!(div_filter.filter(&create_ticker_data(
        Some(FloatingPoint::new(1, 2)),
        Some(FloatingPoint::new(2, 1))
    )));
}

#[test]
fn test_filter_not_pass_one_of() {
    let div_filter: DividendFilter<NoOpFilter> =
        DividendFilter::new(None, FloatingPoint::new(1, 0), FloatingPoint::new(1, 1));
    assert!(!div_filter.filter(&create_ticker_data(
        Some(FloatingPoint::new(1, 1)),
        Some(FloatingPoint::new(1, 0))
    )));

    let div_filter: DividendFilter<NoOpFilter> =
        DividendFilter::new(None, FloatingPoint::new(1, 2), FloatingPoint::new(2, 1));
    assert!(!div_filter.filter(&create_ticker_data(
        Some(FloatingPoint::new(1, 1)),
        Some(FloatingPoint::new(2, 2))
    )));
}

#[test]
fn test_filter_not_pass_both() {
    let div_filter: DividendFilter<NoOpFilter> =
        DividendFilter::new(None, FloatingPoint::new(1, 0), FloatingPoint::new(1, 1));
    assert!(!div_filter.filter(&create_ticker_data(
        Some(FloatingPoint::new(-1, 1)),
        Some(FloatingPoint::new(-1, 0))
    )));

    let div_filter: DividendFilter<NoOpFilter> =
        DividendFilter::new(None, FloatingPoint::new(1, 2), FloatingPoint::new(2, 1));
    assert!(!div_filter.filter(&create_ticker_data(
        Some(FloatingPoint::new(1, 1)),
        Some(FloatingPoint::new(1, 1))
    )));
}

#[test]
fn test_filter_not_pass_no_dividend() {
    let div_filter: DividendFilter<NoOpFilter> =
        DividendFilter::new(None, FloatingPoint::new(1, 0), FloatingPoint::new(1, 1));
    assert!(!div_filter.filter(&create_ticker_data(Some(FloatingPoint::new(10, 2)), None)));

    let div_filter: DividendFilter<NoOpFilter> =
        DividendFilter::new(None, FloatingPoint::new(1, 2), FloatingPoint::new(2, 1));
    assert!(!div_filter.filter(&create_ticker_data(None, Some(FloatingPoint::new(10, 2)))));

    let div_filter: DividendFilter<NoOpFilter> =
        DividendFilter::new(None, FloatingPoint::new(1, 2), FloatingPoint::new(2, 1));
    assert!(!div_filter.filter(&create_ticker_data(None, None)));
}
