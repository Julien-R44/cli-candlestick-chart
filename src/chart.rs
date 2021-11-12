use crate::{info_bar::InfoBar, ChartData, ChartRenderer, YAxis};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Candle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

pub enum CandleType {
    BEARISH,
    BULLISH,
}

impl Candle {
    pub fn get_type(&self) -> CandleType {
        match self.open < self.close {
            true => CandleType::BULLISH,
            false => CandleType::BEARISH,
        }
    }
}

pub struct Chart {
    pub renderer: ChartRenderer,
    pub y_axis: YAxis,
    pub chart_data: ChartData,
    pub info_bar: InfoBar,
}

impl Chart {
    pub fn new(candles: Vec<Candle>) -> Self {
        let renderer = ChartRenderer::new();
        let chart_data = ChartData::new(candles);
        let y_axis = YAxis::new(&chart_data);
        let info_bar = InfoBar::new("APPLE".to_string(), &chart_data);

        Chart {
            renderer,
            y_axis,
            chart_data,
            info_bar,
        }
    }

    pub fn draw(&self) -> () {
        self.renderer.render(self);
    }
}
