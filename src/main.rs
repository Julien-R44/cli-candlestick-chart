use std::error::Error;

use cli_candlestick_chart::{Candle, Chart};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("AAPL.csv")?;

    let mut candles: Vec<Candle> = Vec::new();

    for result in rdr.deserialize() {
        let candle: Candle = result?;
        candles.push(candle);
    }

    let chart = Chart::new(&candles);
    chart.draw();

    Ok(())
}
