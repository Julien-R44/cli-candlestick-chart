use clap::{App, Arg};
use std::str::FromStr;

pub enum ReadingMode {
    Stdin,
    CsvFile,
}

impl FromStr for ReadingMode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Stdin" => Ok(ReadingMode::Stdin),
            "CsvFile" => Ok(ReadingMode::CsvFile),
            _ => Err("no match"),
        }
    }
}

pub struct CliOptions {
    pub reading_mode: ReadingMode,
    pub file_path: Option<String>,
    pub chart_name: Option<String>,
    pub bear_color: Option<String>,
    pub bull_color: Option<String>,
}

pub fn get_args() -> CliOptions {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("READING_MODE")
                .short("r")
                .long("reading-mode")
                .help("Make the program reads and parse candles from stdin.")
                .possible_values(&["stdin", "csv-file", "json-file"])
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("FILE")
                .short("f")
                .long("file")
                .help("File to read candles from, if reading-mode is `*-file.`")
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
        .get_matches();

    return CliOptions {
        reading_mode: match matches.value_of("READING_MODE").unwrap() {
            "stdin" => ReadingMode::Stdin,
            "csv-file" => ReadingMode::CsvFile,
            _ => panic!("Invalid reading mode."),
        },
        file_path: matches.value_of("FILE").map(|s| s.to_string()),
        chart_name: matches.value_of("CHART_NAME").map(|s| s.to_string()),
        bear_color: matches.value_of("BEAR_COLOR").map(|s| s.to_string()),
        bull_color: matches.value_of("BULL_COLOR").map(|s| s.to_string()),
    };
}
