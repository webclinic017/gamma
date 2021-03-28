use super::Broker;
use super::BrokerSlice;

pub struct Backtest {
    
}

struct BacktestSlice {

}

impl Broker for Backtest {
    fn connect() {}
    fn disconnect() {}
}

impl BrokerSlice for BacktestSlice {
    fn cash_available() {}
    fn positions() {}
    fn order_stock() {}
}