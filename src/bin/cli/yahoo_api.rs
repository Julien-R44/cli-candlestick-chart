use cli_candlestick_chart::Candle;

pub fn get_range_from_interval(interval: &str) -> String {
    let output = match interval {
        "1m" => "1d",
        "2m" => "1d",
        "5m" => "1d",
        "15m" => "5d",
        "30m" => "5d",
        "60m" => "5d",
        "90m" => "5d",
        "1h" => "1mo",
        "1d" => "1y",
        "5d" => "1y",
        "1wk" => "1y",
        "1mo" => "1y",
        "3mo" => "1y",
        _ => panic!("Invalid interval"),
    };

    output.to_owned()
}

pub fn get_yahoo_klines(symbol: &str, interval: &str) -> Vec<Candle> {
    let base_url = "https://query1.finance.yahoo.com";

    let range = get_range_from_interval(interval);

    let url = format!(
        "{}/v8/finance/chart/{}?range={}&interval={}",
        base_url, symbol, range, interval
    );

    let client = reqwest::blocking::get(url.as_str()).unwrap();
    let value: serde_json::Value = serde_json::from_str(client.text().unwrap().as_str()).unwrap();

    let nb_candles = value["chart"]["result"][0]["timestamp"]
        .as_array()
        .expect("Ticker seems to be invalid.")
        .len();

    let mut candles: Vec<Candle> = Vec::new();
    for i in 0..nb_candles {
        let base = &value["chart"]["result"][0]["indicators"]["quote"][0];

        let open = base["open"][i].as_f64().unwrap();
        let high = base["high"][i].as_f64().unwrap();
        let low = base["low"][i].as_f64().unwrap();
        let close = base["close"][i].as_f64().unwrap();

        let candle = Candle::new(
            open.to_owned(),
            high.to_owned(),
            low.to_owned(),
            close.to_owned(),
        );
        candles.push(candle);
    }

    candles
}
