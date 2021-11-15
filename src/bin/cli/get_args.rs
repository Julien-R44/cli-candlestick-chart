use clap::{App, Arg};

#[derive(PartialEq)]
pub enum CandlesRetrievalMode {
    Stdin,
    CsvFile,
    Yahoo,
}

pub struct CliOptions {
    pub mode: CandlesRetrievalMode,
    pub file_path: Option<String>,
    pub chart_name: Option<String>,
    pub bear_color: Option<String>,
    pub bull_color: Option<String>,
    pub ticker: Option<String>,
    pub interval: String,
}

pub fn get_args() -> CliOptions {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("MODE")
                .short("m")
                .long("mode")
                .help("Select the method for retrieving the candles.")
                .possible_values(&["stdin", "csv-file", "json-file", "yahoo-fetch"])
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("FILE")
                .short("f")
                .long("file")
                .help("[MODE:*-file] File to read candles from.`")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("CHART_NAME")
                .long("chart-name")
                .help("Sets the chart name.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("BEAR_COLOR")
                .long("bear-color")
                .help("Sets the descending candles color in hexadecimal.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("BULL_COLOR")
                .long("bull-color")
                .help("Sets the ascending candles color in hexadecimal.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("TICKER")
                .long("ticker")
                .takes_value(true)
                .required_if("MODE", "yahoo-fetch")
                .help("[MODE:*-fetch] The broker-side ticker of the asset you want to plot."),
        )
        .arg(
            Arg::with_name("INTERVAL")
                .long("interval")
                .default_value("1d")
                .possible_values(&[
                    "1m", "2m", "5m", "15m", "30m", "60m", "90m", "1h", "1d", "5d", "1wk", "1mo",
                    "3mo",
                ])
                .takes_value(true)
                .help("[MODE:*-fetch] The interval you want to retrieve the candles from the API"),
        )
        .get_matches();

    return CliOptions {
        mode: match matches.value_of("MODE").unwrap() {
            "stdin" => CandlesRetrievalMode::Stdin,
            "csv-file" => CandlesRetrievalMode::CsvFile,
            "yahoo-fetch" => CandlesRetrievalMode::Yahoo,
            _ => panic!("Invalid reading mode."),
        },
        interval: matches.value_of("INTERVAL").unwrap().to_string(),
        ticker: matches.value_of("TICKER").map(|s| s.to_string()),
        file_path: matches.value_of("FILE").map(|s| s.to_string()),
        chart_name: matches.value_of("CHART_NAME").map(|s| s.to_string()),
        bear_color: matches.value_of("BEAR_COLOR").map(|s| s.to_string()),
        bull_color: matches.value_of("BULL_COLOR").map(|s| s.to_string()),
    };
}
