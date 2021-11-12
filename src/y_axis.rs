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
}
