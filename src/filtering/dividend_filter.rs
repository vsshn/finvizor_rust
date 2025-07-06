use crate::filtering::filter_if::SecurityFilterIf;
use crate::stock_related_types::floating_point::FloatingPoint;
use crate::stock_related_types::ticker_data::TickerData;

pub struct DividendFilter {
    wrappee: Option<Box<dyn SecurityFilterIf>>,
    dividend_est: FloatingPoint,
    divident_ttm: FloatingPoint,
}

impl DividendFilter {
    pub fn new(
        wrappee: Option<Box<dyn SecurityFilterIf>>,
        dividend_est: FloatingPoint,
        divident_ttm: FloatingPoint,
    ) -> Self {
        Self {
            wrappee,
            dividend_est,
            divident_ttm,
        }
    }
}

impl SecurityFilterIf for DividendFilter {
    fn filter(&self, ticker_data: &TickerData) -> bool {
        if let Some(wrappee) = &self.wrappee {
            if !wrappee.filter(&ticker_data) {
                return false;
            }
        }

        if ticker_data.dividend_est == None || ticker_data.dividend_ttm == None {
            return false;
        }

        return self.dividend_est <= *ticker_data.dividend_est.as_ref().unwrap()
            && self.divident_ttm <= *ticker_data.dividend_ttm.as_ref().unwrap();
    }
}
