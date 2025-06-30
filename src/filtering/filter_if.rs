use crate::stock_related_types::ticker_data::TickerData;

pub trait SecurityFilterIf {
    fn filter(&self, ticker_data: TickerData) -> bool;
}
