use crate::filtering::filter_if::SecurityFilterIf;
use crate::filtering::pb_filter::PBFilter;
use crate::stock_related_types::floating_point::FloatingPoint;
use crate::stock_related_types::security::Security;
use crate::stock_related_types::ticker_data::TickerData;

fn create_ticker_data(pb: Option<FloatingPoint>) -> TickerData {
    TickerData {
        dividend_ttm: None,
        dividend_est: None,
        security: Security {
            finviz_ticker: "".to_string(),
        },
        price: None,
        pb,
        pe: None,
        ps: None,
        pc: None,
    }
}

#[test]
fn test_filter_pass() {
    let pb_filter: PBFilter = PBFilter::new(None, FloatingPoint::new(1, 2));
    assert!(pb_filter.filter(&create_ticker_data(Some(FloatingPoint::new(1, 1)))));

    let pb_filter: PBFilter = PBFilter::new(None, FloatingPoint::new(1, 2));
    assert!(pb_filter.filter(&create_ticker_data(Some(FloatingPoint::new(2, 1)))));
}

#[test]
fn test_filter_no_pass() {
    let pb_filter: PBFilter = PBFilter::new(None, FloatingPoint::new(1, 2));
    assert!(!pb_filter.filter(&create_ticker_data(Some(FloatingPoint::new(1, 3)))));

    let pb_filter: PBFilter = PBFilter::new(None, FloatingPoint::new(1, 2));
    assert!(!pb_filter.filter(&create_ticker_data(None)));
}
