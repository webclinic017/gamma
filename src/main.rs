use std::{cmp::max, io, process, thread, time::Duration};

use plotters::prelude::*;

use simplelog::*;

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

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
        ]
    ).unwrap();

    let mut c = csv::Csv {filepath: "/Users/david/Desktop/dev/gamma/data/SPY.csv", field: "", data: Vec::new(), idx: 0, end: false};
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

    engine.run();
}
