use crate::stock_related_types::floating_point::FloatingPoint;
use crate::stock_related_types::security::Security;

#[derive(PartialEq, Debug)]
pub struct TickerData {
    // identifier
    pub security: Security,
    pub price: Option<FloatingPoint>,
    // price to book
    pub pb: Option<FloatingPoint>,
    // trailing 12 months dividend in percent
    pub dividend_ttm: Option<FloatingPoint>,
    // dividend estimate in percent
    pub dividend_est: Option<FloatingPoint>,
    // Price to earnings ttm
    pub pe: Option<FloatingPoint>,
}

impl TickerData {
    pub fn new(
        security: Security,
        price: Option<FloatingPoint>,
        pb: Option<FloatingPoint>,
    ) -> Self {
        Self {
            security,
            price,
            pb,
            dividend_ttm: None,
            dividend_est: None,
            pe: None,
        }
    }

    pub fn new_with_dividend(
        security: Security,
        price: Option<FloatingPoint>,
        pb: Option<FloatingPoint>,
        dividend_ttm: Option<FloatingPoint>,
        dividend_est: Option<FloatingPoint>,
    ) -> Self {
        Self {
            security,
            price,
            pb,
            dividend_ttm,
            dividend_est,
            pe: None,
        }
    }

    pub fn default() -> Self {
        TickerData::new(
            Security {
                finviz_ticker: "META".to_string(),
            },
            None,
            None,
        )
    }
}
