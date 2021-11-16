use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use crate::{
    chart_data::ChartData, chart_renderer::ChartRenderer, info_bar::InfoBar,
    volume_pane::VolumePane, y_axis::YAxis,
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Candle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: Option<f64>,
    pub timestamp: Option<i64>,
}

pub enum CandleType {
    Bearish,
    Bullish,
}

impl Candle {
    #[allow(dead_code)]
    pub fn new(
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: Option<f64>,
        timestamp: Option<i64>,
    ) -> Candle {
        Candle {
            open,
            high,
            low,
            close,
            volume,
            timestamp,
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
    pub(crate) volume_pane: VolumePane,
}

impl Chart {
    pub fn new(candles: &[Candle]) -> Self {
        let renderer = ChartRenderer::new();
        let chart_data = Rc::new(RefCell::new(ChartData::new(candles.to_vec())));
        let y_axis = YAxis::new(chart_data.clone());
        let info_bar = InfoBar::new("APPLE".to_string(), chart_data.clone());

        let volume_pane = VolumePane::new(
            chart_data.clone(),
            (chart_data.borrow().terminal_size.1 / 6) as i64,
        );

        chart_data
            .borrow_mut()
            .get_mut()
            .compute_height(&volume_pane);

        Chart {
            renderer,
            y_axis,
            chart_data,
            info_bar,
            volume_pane,
        }
    }

    /// Draws the chart by outputting multiples strings in the terminal.
    pub fn draw(&self) {
        self.renderer.render(self);
    }

    /// Set the name of the chart in the info bar.
    pub fn set_name(&mut self, name: String) {
        self.info_bar.name = name;
    }

    /// Set the color of the bearish candle
    /// The default color is  (234, 74, 90).
    pub fn set_bear_color(&mut self, r: u8, g: u8, b: u8) {
        self.renderer.bearish_color = (r, g, b);
    }

    /// Set the color of the bullish candle
    /// The default color is  (52, 208, 88).
    pub fn set_bull_color(&mut self, r: u8, g: u8, b: u8) {
        self.renderer.bullish_color = (r, g, b);
    }

    /// Sets the color of the volume when the candle is bearish.
    /// The default color is  (234, 74, 90).
    pub fn set_vol_bear_color(&mut self, r: u8, g: u8, b: u8) {
        self.volume_pane.bearish_color = (r, g, b);
    }

    /// Sets the color of the volume when the candle is bullish.
    /// The default color is  (52, 208, 88).
    pub fn set_vol_bull_color(&mut self, r: u8, g: u8, b: u8) {
        self.volume_pane.bullish_color = (r, g, b);
    }

    /// Hide or show the volume pane.
    pub fn set_volume_pane_enabled(&mut self, enabled: bool) {
        self.volume_pane.enabled = enabled;
    }

    /// Set the character for drawing the volume bars.
    pub fn set_volume_pane_unicode_fill(&mut self, unicode_fill: char) {
        self.volume_pane.unicode_fill = unicode_fill;
    }

    /// Set the volume pane height.
    /// Default is 1/6 of the terminal height.
    pub fn set_volume_pane_height(&mut self, height: i64) {
        self.volume_pane.height = height;
    }
}
