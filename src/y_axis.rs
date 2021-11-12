use crate::chart_data::ChartData;

pub struct YAxis {
    chart_data: ChartData,
    width: i64,
    margin_right: i64,
    max_len_characteristic: i64,
}

impl YAxis {
    pub fn new(chart_data: &ChartData) -> YAxis {
        let cloned_chart_data = chart_data.clone();

        let max_len_characteristic = chart_data.max_value.ceil().to_string().len() as i64;

        YAxis {
            chart_data: cloned_chart_data,
            width: 5,
            margin_right: 4,
            max_len_characteristic,
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
            _ => self.render_empty(),
        }
    }

    fn render_tick(&self, y: u16) -> String {
        let min_value = self.chart_data.min_value;
        let max_value = self.chart_data.max_value;
        let height = self.chart_data.terminal_size.1;

        let price = min_value + (y as f64 * (max_value - min_value) / height as f64);
        let cell_min_length = (self.max_len_characteristic + 3) as usize;

        format!(
            "{0:<cell_min_length$.2} │┈{margin}",
            price,
            cell_min_length = cell_min_length,
            margin = " ".repeat(self.margin_right as usize)
        )
    }

    fn render_empty(&self) -> String {
        let cell = " ".repeat((self.max_len_characteristic + 4) as usize);
        let margin = " ".repeat((self.margin_right + 1).try_into().unwrap());

        format!("{}│{}", cell, margin)
    }
}
