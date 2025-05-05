pub enum Value {
    Positive(u64),
    Negative(u64),
    Zero,
}

pub struct Price {
    mantissa: Value,
    exp: i8,
}

pub struct TickerData {
    price: Price,
    pe: f32,
    ps: f32,
    ticker: String,
}

impl Price {
    pub fn new(mantissa: Value, exp: i8) -> Self {
        Self { mantissa, exp }
    }
}

impl TickerData {
    pub fn new(price: Price, pe: f32, ps: f32, ticker: String) -> Self {
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
            self.ticker, self.pe, self.ps
        );
    }
}
