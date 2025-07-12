use crate::filtering::dividend_filter::DividendFilter;
use crate::filtering::filter_if::SecurityFilterIf;
use crate::filtering::pb_filter::PBFilter;
use crate::stock_related_types::floating_point::FloatingPoint;
use crate::stock_related_types::security::Security;
use crate::stock_related_types::ticker_data::TickerData;

fn create_ticker_data(
    pb: Option<FloatingPoint>,
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
        pb,
        pe: None,
        ps: None,
        pc: None,
    }
}

fn create_optional_float(mantissa: i64, exponent: i8) -> Option<FloatingPoint> {
    Some(FloatingPoint::new(mantissa, exponent))
}

#[test]
fn test_compound_filter() {
    let pb_filter: Box<PBFilter> = Box::new(PBFilter::new(None, FloatingPoint::new(3, 0)));
    let compound_filter = DividendFilter::new(
        Some(pb_filter),
        FloatingPoint::new(5, 0),
        FloatingPoint::new(5, 0),
    );

    assert!(compound_filter.filter(&create_ticker_data(
        create_optional_float(2, 0),
        create_optional_float(6, 0),
        create_optional_float(6, 0)
    )));

    // Just pb doesn't filter
    assert!(!compound_filter.filter(&create_ticker_data(
        create_optional_float(4, 0),
        create_optional_float(6, 0),
        create_optional_float(6, 0)
    )));

    // just dividends don't filter
    assert!(!compound_filter.filter(&create_ticker_data(
        create_optional_float(2, 0),
        create_optional_float(3, 0),
        create_optional_float(6, 0)
    )));

    // Both don't filter
    assert!(!compound_filter.filter(&create_ticker_data(
        create_optional_float(6, 0),
        create_optional_float(3, 0),
        create_optional_float(3, 0)
    )))
}
