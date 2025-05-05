#[derive(PartialEq, Debug)]
pub enum Value {
    Positive(u64),
    Negative(u64),
    Zero,
}

#[derive(PartialEq, Debug)]
pub struct Price {
    mantissa: Value,
    exp: i8,
}

#[derive(Debug, PartialEq)]
pub struct TickerData {
    price: Option<Price>,
    pe: Option<f32>,
    ps: Option<f32>,
    ticker: String,
}

impl Price {
    pub fn new(mantissa: Value, exp: i8) -> Self {
        Self { mantissa, exp }
    }
}

impl TickerData {
    pub fn new(price: Option<Price>, pe: Option<f32>, ps: Option<f32>, ticker: String) -> Self {
        Self {
            price,
            pe,
            ps,
            ticker,
        }
    }

    pub fn print(&self) {
        println!(
            "Ticker: {}, P/E: {}, P/S: {}",
            self.ticker,
            self.pe.unwrap(),
            self.ps.unwrap()
        );
    }
}
