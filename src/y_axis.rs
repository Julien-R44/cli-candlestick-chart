use crate::chart_data::ChartData;

pub struct YAxis {
    chart_data: ChartData,
}

impl YAxis {
    pub fn new(chart_data: &ChartData) -> YAxis {
        YAxis {
            chart_data: chart_data.clone(),
        }
    }

    pub fn price_to_height(&self, price: f64) -> f64 {
        let height = self.chart_data.terminal_size.1;

        let min_value = self.chart_data.min_value;
        let max_value = self.chart_data.max_value;

        (price - min_value) / (max_value - min_value) * height as f64
    }

    pub fn render_line(&self, y: u16) -> String {
        match y % 4 {
            0 => self.render_tick(y),
            _ => self.render_empty(y),
        }
    }

    fn render_tick(&self, y: u16) -> String {
        let min_value = self.chart_data.min_value;
        let max_value = self.chart_data.max_value;
        let height = self.chart_data.terminal_size.1;

        let output = format!(
            "{:.2} │┈   ",
            min_value + (y as f64 * (max_value - min_value) / height as f64)
        );

        return output;
    }

    fn render_empty(&self, y: u16) -> String {
        "       │    ".to_string()
    }
}
