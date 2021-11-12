use colored::Colorize;

use crate::{chart::Chart, chart_data::ChartData};

pub struct InfoBar {
    pub name: String,
    chart_data: ChartData,
}

impl InfoBar {
    pub fn new(name: String, chart_data: &ChartData) -> InfoBar {
        InfoBar {
            name,
            chart_data: chart_data.clone(),
        }
    }

    pub fn render(&self, chart: &Chart) -> String {
        let candles = self.chart_data.candles.clone();
        let mut output_str = String::new();

        output_str += "\n";
        output_str += &"─".repeat(candles.len() + chart.y_axis.width as usize);
        output_str += "\n";

        let mut avg_format = format!("{:.2}", self.chart_data.average);

        if self.chart_data.last_price > self.chart_data.average {
            avg_format = avg_format.bold().red().to_string();
        } else if self.chart_data.last_price < self.chart_data.average {
            avg_format = avg_format.bold().green().to_string();
        } else {
            avg_format = avg_format.bold().yellow().to_string();
        }

        output_str += &format!(
            "{:>width$} | Price: {price} | Highest: {high} | Lowest: {low} | Var.: {var} | Avg.: {avg}",
            self.name,
            width = chart.y_axis.width as usize,
            high = format!("{:.2}", self.chart_data.max_value).green().bold(),
            low = format!("{:.2}", self.chart_data.min_value).red().bold(),
            var = format!("↗ +{:.2}%", self.chart_data.variation).green().bold(),
            avg = avg_format,
            price = format!("{:.2}", self.chart_data.last_price).green().bold(),
        );

        return output_str;
    }
}
