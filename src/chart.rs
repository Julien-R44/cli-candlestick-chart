use std::{cell::RefCell, rc::Rc};

use serde::Deserialize;

use crate::{
    chart_data::ChartData, chart_renderer::ChartRenderer, info_bar::InfoBar, y_axis::YAxis
};

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
    pub(crate) renderer: ChartRenderer,
    pub(crate) y_axis: YAxis,
    pub(crate) chart_data: Rc<RefCell<ChartData>>,
    pub(crate) info_bar: InfoBar,
}

impl Chart {
    pub fn new(candles: &[Candle]) -> Self {
        let renderer = ChartRenderer::new();
        let chart_data = Rc::new(RefCell::new(ChartData::new(candles.to_vec())));
        let y_axis = YAxis::new(chart_data.clone());
        let info_bar = InfoBar::new("APPLE".to_string(), chart_data.clone());

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

    pub fn set_name(&mut self, name: String) {
        self.info_bar.name = name;
    }

    pub fn set_bear_color(&mut self, r: u8, g: u8, b: u8) {
        self.renderer.bearish_color = (r, g, b);
    }

    pub fn set_bull_color(&mut self, r: u8, g: u8, b: u8) {
        self.renderer.bullish_color = (r, g, b);
    }
}
