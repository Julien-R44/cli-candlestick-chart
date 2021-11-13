use std::error::Error;

use cli_candlestick_chart::{Candle, Chart};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("BTC-USD.csv")?;

    let mut candles: Vec<Candle> = Vec::new();

    for result in rdr.deserialize() {
        let candle: Candle = result?;
        candles.push(candle);
    }

    let mut chart = Chart::new(&candles);

    chart.set_name(String::from("BTC/USDT"));

    // Set custom green color
    chart.set_bear_color(1, 205, 254);

    // Set custom red color
    chart.set_bull_color(255, 107, 153);

    chart.draw();

    // Set the chart title

    Ok(())
}
