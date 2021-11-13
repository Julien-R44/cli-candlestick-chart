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

    // Set the chart title
    chart.set_name(String::from("BTC/USDT"));

    // Set customs colors
    chart.set_bear_color(1, 205, 254);
    chart.set_bull_color(255, 107, 153);

    chart.draw();

    Ok(())
}
