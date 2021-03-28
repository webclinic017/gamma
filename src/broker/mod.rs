pub mod backtest;
pub mod etrade;

pub trait Broker {
    fn connect();
    fn disconnect();
}

pub trait BrokerSlice {
    fn cash_available();
    fn positions();
    fn order_stock();
}