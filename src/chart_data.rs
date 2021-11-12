use crate::chart::Candle;
use terminal_size::terminal_size;

#[derive(Debug, Clone)]
pub struct ChartData {
    pub candles: Vec<Candle>,
    pub min_value: f64,
    pub max_value: f64,
    pub terminal_size: (u16, u16),
}

impl ChartData {
    pub fn new(candles: Vec<Candle>) -> ChartData {
        let (w, h) = terminal_size().unwrap();
        let mut chart_data = ChartData {
            candles: Vec::new(),
            min_value: 0.0,
            max_value: 0.0,
            terminal_size: (w.0, h.0),
        };

        chart_data.set_candles(candles);
        chart_data
    }

    pub fn set_candles(&mut self, candles: Vec<Candle>) {
        self.candles = candles.clone();
        self.compute_min_and_max_values();
    }

    fn compute_min_and_max_values(&mut self) {
        let mut sorted_candles = self.candles.clone();

        sorted_candles.sort_by(|a, b| a.low.partial_cmp(&b.low).unwrap());
        self.min_value = sorted_candles.first().unwrap().low;
        sorted_candles.sort_by(|a, b| b.high.partial_cmp(&a.high).unwrap());
        self.max_value = sorted_candles.first().unwrap().high;
    }
}
