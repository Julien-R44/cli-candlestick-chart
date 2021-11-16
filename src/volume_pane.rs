use crate::{chart_data::ChartData, Candle, CandleType};
use colored::Colorize;
use std::{cell::RefCell, rc::Rc};

pub struct VolumePane {
    pub chart_data: Rc<RefCell<ChartData>>,
    pub height: i64,
    pub enabled: bool,
    pub bearish_color: (u8, u8, u8),
    pub bullish_color: (u8, u8, u8),
    pub unicode_fill: char,
}

impl VolumePane {
    pub fn new(chart_data: Rc<RefCell<ChartData>>, height: i64) -> Self {
        let candle_set_has_volume = chart_data
            .borrow()
            .visible_candle_set
            .candles
            .iter()
            .any(|candle| candle.volume.unwrap_or_default() > 0.0);

        Self {
            chart_data,
            height,
            enabled: candle_set_has_volume,
            bullish_color: (52, 208, 88),
            bearish_color: (234, 74, 90),
            unicode_fill: '┃',
        }
    }

    fn colorize(&self, candle_type: &CandleType, string: &str) -> String {
        let (ar, ag, ab) = self.bearish_color;
        let (br, bg, bb) = self.bullish_color;

        match candle_type {
            CandleType::Bearish => format!("{}", string.truecolor(ar, ag, ab)),
            CandleType::Bullish => format!("{}", string.truecolor(br, bg, bb)),
        }
    }

    pub fn render(&self, candle: &Candle, y: i64) -> String {
        let max_volume = self.chart_data.borrow().visible_candle_set.max_volume;
        let volume = candle.volume.unwrap_or_default();

        let volume_percent_of_max = volume / max_volume;
        let b = volume_percent_of_max * self.height as f64;

        if y < b.floor() as i64 {
            return self.colorize(&candle.get_type(), &self.unicode_fill.to_string());
        }

        " ".to_string()
    }
}
