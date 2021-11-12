use std::error::Error;

mod chart;
use chart::*;

mod chart_renderer;
use chart_renderer::*;

mod y_axis;
use y_axis::*;

mod chart_data;
use chart_data::*;

mod info_bar;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("AAPL.csv")?;

    let mut candles: Vec<Candle> = Vec::new();

    for result in rdr.deserialize() {
        let candle: Candle = result?;
        candles.push(candle);
    }

    let chart = Chart::new(candles);
    chart.draw();

    Ok(())
}
