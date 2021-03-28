pub mod broker;
use broker::backtest;

pub mod strategy;
use strategy::teststrat;

pub mod datasource;
use datasource::DataSource;
use datasource::csv;

pub mod engine;

fn main() {
    let mut c = csv::Csv {filepath: "/Users/david/Desktop/dev/gamma/data/SPY.csv", field: "", data: Vec::new(), idx: 0};
    let mut t = teststrat::TestStrat {};
    let mut b = backtest::Backtest {};

    c.connect();

    let mut engine = engine::Engine {
        broker: &mut b,
        strategy: &mut t,
        datasource: &mut c
    };

    engine.run();
}
