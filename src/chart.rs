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
    Bearish,
    Bullish,
}

impl Candle {
    #[allow(dead_code)]
    pub fn new(open: f64, high: f64, low: f64, close: f64) -> Candle {
        Candle {
            open,
            high,
            low,
            close,
        }
    }

    pub fn get_type(&self) -> CandleType {
        match self.open < self.close {
            true => CandleType::Bullish,
            false => CandleType::Bearish,
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

    pub fn draw(&self) {
        self.renderer.render(self);
    }
}
