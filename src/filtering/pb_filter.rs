use crate::filtering::filter_if::SecurityFilterIf;
use crate::stock_related_types::floating_point::FloatingPoint;
use crate::stock_related_types::ticker_data::TickerData;

pub struct PBFilter {
    wrappee: Option<Box<dyn SecurityFilterIf>>,
    pb_trshld: FloatingPoint,
}

impl PBFilter {
    pub fn new(wrappee: Option<Box<dyn SecurityFilterIf>>, pb_trshld: FloatingPoint) -> Self {
        Self { wrappee, pb_trshld }
    }
}

impl SecurityFilterIf for PBFilter {
    fn filter(&self, ticker_data: &TickerData) -> bool {
        if let Some(wrappee) = &self.wrappee {
            if !wrappee.filter(&ticker_data) {
                return false;
            }
        }

        if ticker_data.pb == None {
            return false;
        }

        return self.pb_trshld >= *ticker_data.pb.as_ref().unwrap();
    }
}
