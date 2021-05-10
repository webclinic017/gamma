use plotters::prelude::*;


pub struct SimpleChart {
    title: String,
    data: Vec<(&'static str, Vec<(f32, f32)>)>
}

impl SimpleChart {
    pub fn new(title: String) -> SimpleChart {
        return SimpleChart {
            title: title,
            data: Vec::new()
        };
    }

    pub fn add_series(&mut self, name: &'static str, series: Vec<(f32, f32)>) {
        self.data.push((name, series));
    }

    pub fn create(&mut self) {
        // TODO calculate min_equity
        // TODO calculate max_equity 

        let root = SVGBackend::new("plot.svg", (1000, 500)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("equity", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_cartesian_2d(0.0..s, min_equity..max_equity)?;

        chart.configure_mesh().draw()?;


        for (name, series) in self.data.iter() {
            chart
                .draw_series(LineSeries::new(
                    data,
                    &RED,
                ))?
                .label(name)
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        }

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;
    }
}