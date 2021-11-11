use colored::*;
use serde::Deserialize;
use std::error::Error;
use terminal_size::{terminal_size, Height, Width};

#[derive(Debug, Deserialize, Clone)]
struct Candle {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

enum CandleType {
    BEARISH,
    BULLISH,
}

impl Candle {
    fn get_type(&self) -> CandleType {
        let is_bullish = self.open < self.close;
        match is_bullish {
            true => CandleType::BULLISH,
            false => CandleType::BEARISH,
        }
    }
}

struct Chart {
    candles: Vec<Candle>,
    min_value: f64,
    max_value: f64,
}

const HEIGHT: i32 = 30;

impl Chart {
    const UNICODE_VOID: char = ' ';
    const UNICODE_BODY: char = '┃';
    const UNICODE_HALF_BODY_BOTTOM: char = '╻';
    const UNICODE_HALF_BODY_TOP: char = '╹';
    const UNICODE_WICK: char = '│';
    const UNICODE_TOP: char = '╽';
    const UNICODE_BOTTOM: char = '╿';
    const UNICODE_UPPER_WICK: char = '╷';
    const UNICODE_LOWER_WICK: char = '╵';

    fn new(candles: Vec<Candle>) -> Self {
        let mut sorted_candles = candles.clone();

        sorted_candles.sort_by(|a, b| a.low.partial_cmp(&b.low).unwrap());
        let min_value = sorted_candles.first().unwrap().low;
        sorted_candles.sort_by(|a, b| b.high.partial_cmp(&a.high).unwrap());
        let max_value = sorted_candles.first().unwrap().high;

        Chart {
            candles: candles,
            min_value,
            max_value,
        }
    }

    fn price_to_height(&self, price: f64) -> f64 {
        (price - self.min_value) / (self.max_value - self.min_value) * HEIGHT as f64
    }

    fn render_candle(&self, candle: &Candle, y: i32) -> String {
        let height_unit = y as f64;
        let high_y = self.price_to_height(candle.high);
        let low_y = self.price_to_height(candle.low);
        let max_y = self.price_to_height(candle.open.max(candle.close));
        let min_y = self.price_to_height(candle.close.min(candle.open));

        let mut output = Chart::UNICODE_VOID;

        if high_y.ceil() >= height_unit && height_unit >= max_y.floor() {
            if max_y - height_unit > 0.75 {
                output = Chart::UNICODE_BODY;
            } else if (max_y - height_unit) > 0.25 {
                if (high_y - height_unit) > 0.75 {
                    output = Chart::UNICODE_TOP;
                } else {
                    output = Chart::UNICODE_HALF_BODY_BOTTOM;
                }
            } else {
                if (high_y - height_unit) > 0.75 {
                    output = Chart::UNICODE_WICK;
                } else if (high_y - height_unit) > 0.25 {
                    output = Chart::UNICODE_UPPER_WICK;
                }
            }
        } else if max_y.floor() >= height_unit && height_unit >= min_y.ceil() {
            output = Chart::UNICODE_BODY;
        } else if min_y.ceil() >= height_unit && height_unit >= low_y.floor() {
            if (min_y - height_unit) < 0.25 {
                output = Chart::UNICODE_BODY;
            } else if (min_y - height_unit) < 0.75 {
                if (low_y - height_unit) < 0.25 {
                    output = Chart::UNICODE_BOTTOM;
                } else {
                    output = Chart::UNICODE_HALF_BODY_TOP;
                }
            } else {
                if low_y - height_unit < 0.25 {
                    output = Chart::UNICODE_WICK;
                } else if low_y - height_unit < 0.75 {
                    output = Chart::UNICODE_LOWER_WICK;
                }
            }
        }

        let candle_type = candle.get_type();
        match candle_type {
            CandleType::BULLISH => output.to_string().green().to_string(),
            CandleType::BEARISH => output.to_string().red().to_string(),
        }
    }

    pub fn draw(&self) -> () {
        let mut output_str = "".to_owned();

        for y in (1..HEIGHT).rev() {
            output_str += "\n";

            if y % 4 == 0 {
                let y_axis = format!(
                    "{:.2} │┈   ",
                    self.min_value + (y as f64 * (self.max_value - self.min_value) / HEIGHT as f64)
                );

                output_str += &y_axis;
            } else {
                output_str += "       │    ";
            }

            for candle in self.candles.iter() {
                output_str += &self.render_candle(candle, y);
            }
        }

        println!("{}", output_str)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("TSLA.csv")?;

    let mut candles: Vec<Candle> = Vec::new();

    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        println!("Your terminal is {} cols wide and {} lines tall", w, h);
    } else {
        println!("Unable to get terminal size");
    }

    for result in rdr.deserialize() {
        let candle: Candle = result?;
        candles.push(candle);
    }

    let chart = Chart::new(candles);

    chart.draw();

    Ok(())
}
