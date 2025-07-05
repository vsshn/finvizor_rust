use std::ops::Div;

use crate::filtering::dividend_filter::DividendFilter;
use crate::filtering::filter_if::{NoOpFilter, SecurityFilterIf};
use crate::stock_related_types::floating_point::FloatingPoint;
use crate::stock_related_types::security::Security;
use crate::stock_related_types::ticker_data::TickerData;

#[test]
fn test_conversion_normal() {
    let div_filter: DividendFilter<NoOpFilter> =
        DividendFilter::new(None, FloatingPoint::new(1, 0), FloatingPoint::new(1, 1));
    assert!(div_filter.filter(&TickerData {
        dividend_ttm: Some(FloatingPoint::new(1, 2)),
        dividend_est: Some(FloatingPoint::new(2, 2)),
        security: Security {
            finviz_ticker: "".to_string()
        },
        price: None,
        pb: None,
        pe: None,
    }));
    let further_div_filter = DividendFilter::new(
        Some(div_filter),
        FloatingPoint::new(2, 2),
        FloatingPoint::new(2, 2),
    );
    assert!(!further_div_filter.filter(&TickerData {
        dividend_ttm: Some(FloatingPoint::new(1, 2)),
        dividend_est: Some(FloatingPoint::new(2, 2)),
        security: Security {
            finviz_ticker: "".to_string()
        },
        price: None,
        pb: None,
        pe: None,
    }));
}
