use crate::stock_related_types::floating_point::FloatingPoint;
use crate::stock_related_types::security::Security;

#[derive(PartialEq, Debug)]
pub struct TickerData {
    // identifier
    pub security: Security,
    pub price: Option<FloatingPoint>,
    // price to book
    pub pb: Option<FloatingPoint>,
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
