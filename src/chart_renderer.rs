use crate::{y_axis::YAxis, Candle, CandleType, Chart};
use colored::*;

pub struct ChartRenderer {}

impl ChartRenderer {
    const UNICODE_VOID: char = ' ';
    const UNICODE_BODY: char = '┃';
    const UNICODE_HALF_BODY_BOTTOM: char = '╻';
    const UNICODE_HALF_BODY_TOP: char = '╹';
    const UNICODE_WICK: char = '│';
    const UNICODE_TOP: char = '╽';
    const UNICODE_BOTTOM: char = '╿';
    const UNICODE_UPPER_WICK: char = '╷';
    const UNICODE_LOWER_WICK: char = '╵';

    pub const MARGIN_TOP: i64 = 3;

    pub fn new() -> ChartRenderer {
        ChartRenderer {}
    }

    fn render_candle(&self, candle: &Candle, y: i32, y_axis: &YAxis) -> String {
        let height_unit = y as f64;
        let high_y = y_axis.price_to_height(candle.high);
        let low_y = y_axis.price_to_height(candle.low);
        let max_y = y_axis.price_to_height(candle.open.max(candle.close));
        let min_y = y_axis.price_to_height(candle.close.min(candle.open));

        let mut output = ChartRenderer::UNICODE_VOID;

        if high_y.ceil() >= height_unit && height_unit >= max_y.floor() {
            if max_y - height_unit > 0.75 {
                output = ChartRenderer::UNICODE_BODY;
            } else if (max_y - height_unit) > 0.25 {
                if (high_y - height_unit) > 0.75 {
                    output = ChartRenderer::UNICODE_TOP;
                } else {
                    output = ChartRenderer::UNICODE_HALF_BODY_BOTTOM;
                }
            } else if (high_y - height_unit) > 0.75 {
                output = ChartRenderer::UNICODE_WICK;
            } else if (high_y - height_unit) > 0.25 {
                output = ChartRenderer::UNICODE_UPPER_WICK;
            }
        } else if max_y.floor() >= height_unit && height_unit >= min_y.ceil() {
            output = ChartRenderer::UNICODE_BODY;
        } else if min_y.ceil() >= height_unit && height_unit >= low_y.floor() {
            if (min_y - height_unit) < 0.25 {
                output = ChartRenderer::UNICODE_BODY;
            } else if (min_y - height_unit) < 0.75 {
                if (low_y - height_unit) < 0.25 {
                    output = ChartRenderer::UNICODE_BOTTOM;
                } else {
                    output = ChartRenderer::UNICODE_HALF_BODY_TOP;
                }
            } else if low_y - height_unit < 0.25 {
                output = ChartRenderer::UNICODE_WICK;
            } else if low_y - height_unit < 0.75 {
                output = ChartRenderer::UNICODE_LOWER_WICK;
            }
        }

        let candle_type = candle.get_type();
        match candle_type {
            CandleType::Bullish => output.to_string().green().to_string(),
            CandleType::Bearish => output.to_string().red().to_string(),
        }
    }

    pub fn render(&self, chart: &Chart) {
        let mut output_str = "".to_owned();

        let chart_data = chart.chart_data.borrow();

        for y in (1..chart_data.height as u16).rev() {
            output_str += "\n";

            output_str += &chart.y_axis.render_line(y);

            for candle in chart_data.visible_candle_set.candles.iter() {
                output_str += &self.render_candle(candle, y.into(), &chart.y_axis);
            }
        }

        output_str += &chart.info_bar.render();

        println!("{}", output_str)
    }
}

impl Default for ChartRenderer {
    fn default() -> Self {
        Self::new()
    }
}
