use crate::stock_related_types::ticker_data::TickerData;

pub trait SecurityFilterIf {
    fn filter(&self, ticker_data: &TickerData) -> bool;
}

pub struct NoOpFilter;

impl SecurityFilterIf for NoOpFilter {
    fn filter(&self, _ticker_data: &TickerData) -> bool {
        return true;
    }
}
