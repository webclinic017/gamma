use std::{cmp::max, io, process, thread, time::Duration};

use plotters::prelude::*;

pub mod broker;

use broker::backtest;

pub mod strategy;
use strategy::teststrat;

pub mod datasource;
use datasource::DataSource;
use datasource::csv;

pub mod engine;

pub mod indicator;
use indicator::Sma;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut c = csv::Csv {filepath: "/Users/david/Desktop/dev/gamma/data/SPY.csv", field: "", data: Vec::new(), idx: 0};
    c.connect();

    let mut t = teststrat::TestStrat {
        previous: 0 as f64
    };

    let mut b = backtest::Backtest::new(1000.0);

    let mut engine = engine::Engine {
        broker: &mut b,
        strategy: &mut t,
        datasource: &mut c
    };

    //engine.run();

    let mut data: Vec<(f32, f32)> = Vec::new();
    
    let mut s = 0 as f32;
    let mut min_equity = f32::MAX;
    let mut max_equity = f32::MIN;


    let mut stop = false;
    while engine.step() && !stop {
        let new_equity = engine.equity() as f32;
        if new_equity < min_equity { min_equity = new_equity; }
        if new_equity > max_equity { max_equity = new_equity; }
        data.push((s, new_equity as f32));
        s += 1.0;
    }

    let root = SVGBackend::new("plot.svg", (1000, 500)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("equity", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0.0..s, min_equity..max_equity)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            data,
            &RED,
        ))?
        .label("equity")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 100, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
