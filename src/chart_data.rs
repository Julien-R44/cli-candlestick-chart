use crate::chart::Candle;
use terminal_size::terminal_size;

#[derive(Debug, Clone)]
pub struct ChartData {
    pub candles: Vec<Candle>,
    pub min_value: f64,
    pub max_value: f64,
    pub variation: f64,
    pub average: f64,
    pub last_price: f64,
    pub terminal_size: (u16, u16),
}

impl ChartData {
    pub fn new(candles: Vec<Candle>) -> ChartData {
        let (w, h) = terminal_size().unwrap();
        let mut chart_data = ChartData {
            candles: Vec::new(),
            min_value: 0.0,
            max_value: 0.0,
            variation: 0.0,
            average: 0.0,
            last_price: 0.0,
            terminal_size: (w.0, h.0),
        };

        chart_data.set_candles(candles);
        chart_data
    }

    pub fn set_candles(&mut self, candles: Vec<Candle>) {
        self.candles = candles.clone();
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
        let first_candle = self.candles.first().unwrap();
        let last_candle = self.candles.last().unwrap();

        self.variation = (last_candle.close - first_candle.close) / first_candle.close;
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
