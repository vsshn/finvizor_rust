use crate::filtering::filter_if::SecurityFilterIf;
use crate::stock_related_types::floating_point::FloatingPoint;
use crate::stock_related_types::ticker_data::TickerData;

pub struct DividendFilter<T: SecurityFilterIf> {
    wrappee: T,
    dividend_est: FloatingPoint,
    divident_ttm: FloatingPoint,
}

impl<T: SecurityFilterIf> DividendFilter<T> {
    pub fn new(wrappee: T, dividend_est: FloatingPoint, divident_ttm: FloatingPoint) -> Self {
        Self {
            wrappee,
            dividend_est,
            divident_ttm,
        }
    }
}

impl<T: SecurityFilterIf> SecurityFilterIf for DividendFilter<T> {
    fn filter(&self, ticker_data: TickerData) -> bool {
        if !self.wrappee.filter(ticker_data) {
            return false;
        }
        return true;
    }
}
