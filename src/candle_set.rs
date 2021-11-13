use crate::Candle;

#[derive(Debug, Clone)]
pub struct CandleSet {
    pub candles: Vec<Candle>,
    pub min_value: f64,
    pub max_value: f64,
    pub variation: f64,
    pub average: f64,
    pub last_price: f64,
}

impl CandleSet {
    pub fn new(candles: Vec<Candle>) -> CandleSet {
        let mut cs = CandleSet {
            candles: Vec::new(),
            min_value: 0.0,
            max_value: 0.0,
            variation: 0.0,
            average: 0.0,
            last_price: 0.0,
        };

        cs.set_candles(candles);

        cs
    }

    pub fn set_candles(&mut self, candles: Vec<Candle>) {
        self.candles = candles;

        if self.candles.is_empty() {
            return;
        }

        self.compute_min_and_max_values();
        self.compute_variation();
        self.compute_average();
        self.compute_last_price();
    }

    fn compute_last_price(&mut self) {
        let last_candle = self.candles.last().unwrap();
        self.last_price = last_candle.close;
    }

    fn compute_variation(&mut self) {
        let open = self.candles.first().unwrap().open;
        let close = self.candles.last().unwrap().close;

        self.variation = ((close - open) / open) * 100.0;
    }

    fn compute_average(&mut self) {
        let mut sum = 0.0;
        for candle in &self.candles {
            sum += candle.close;
        }

        self.average = sum / self.candles.len() as f64;
    }

    fn compute_min_and_max_values(&mut self) {
        let mut sorted_candles = self.candles.clone();

        sorted_candles.sort_by(|a, b| a.low.partial_cmp(&b.low).unwrap());
        self.min_value = sorted_candles.first().unwrap().low;
        sorted_candles.sort_by(|a, b| b.high.partial_cmp(&a.high).unwrap());
        self.max_value = sorted_candles.first().unwrap().high;
    }
}
