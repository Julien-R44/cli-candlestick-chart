use std::error::Error;

use cli_candlestick_chart::{Candle, Chart};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
struct BinanceKlinesItem {
    open_time: u64,
    open: String,
    high: String,
    low: String,
    close: String,
    volume: String,
    close_time: u64,
    quote_asset_volume: String,
    number_of_trades: u64,
    taker_buy_base_asset_volume: String,
    taker_buy_quote_asset_volume: String,
    ignore: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let candles =
        reqwest::blocking::get("https://api.binance.com/api/v1/klines?symbol=BTCUSDT&interval=1h")?
            .json::<Vec<BinanceKlinesItem>>()?
            .iter()
            .map(|candle| {
                Candle::new(
                    candle.open.parse::<f64>().unwrap(),
                    candle.high.parse::<f64>().unwrap(),
                    candle.low.parse::<f64>().unwrap(),
                    candle.close.parse::<f64>().unwrap(),
                )
            })
            .collect::<Vec<Candle>>();

    let mut chart = Chart::new(&candles);

    // Set the chart title
    chart.set_name(String::from("BTC/USDT"));

    // Set customs colors
    chart.set_bear_color(1, 205, 254);
    chart.set_bull_color(255, 107, 153);

    chart.draw();

    Ok(())
}
