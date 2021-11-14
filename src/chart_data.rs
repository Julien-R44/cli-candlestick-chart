use crate::{
    candle_set::CandleSet, chart::Candle, chart_renderer::ChartRenderer, info_bar::InfoBar, y_axis::YAxis,
};
use terminal_size::terminal_size;

#[derive(Debug, Clone)]
pub struct ChartData {
    pub main_candle_set: CandleSet,
    pub visible_candle_set: CandleSet,
    pub terminal_size: (u16, u16),
    pub height: i64,
}

impl ChartData {
    pub fn new(candles: Vec<Candle>) -> ChartData {
        let (w, h) = terminal_size().unwrap();

        let mut chart_data = ChartData {
            main_candle_set: CandleSet::new(candles),
            visible_candle_set: CandleSet::new(Vec::new()),
            terminal_size: (w.0, h.0),
            height: h.0 as i64 - ChartRenderer::MARGIN_TOP - InfoBar::HEIGHT,
        };

        chart_data.compute_visible_candles();
        chart_data
    }

    pub fn compute_visible_candles(&mut self) {
        let term_width = self.terminal_size.0 as usize as i64;
        let nb_candles = self.main_candle_set.candles.len();

        let nb_visible_candles = term_width - YAxis::WIDTH;

        self.visible_candle_set.set_candles(
            self.main_candle_set
                .candles
                .iter()
                .skip((nb_candles as i64 - nb_visible_candles as i64).max(0) as usize)
                .cloned()
                .collect::<Vec<Candle>>(),
        );
    }
}
