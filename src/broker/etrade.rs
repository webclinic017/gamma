use super::Broker;
use super::BrokerSlice;

struct ETrade {

}

struct ETradeSlice {

}

impl Broker for ETrade {
    fn connect() {}
    fn disconnect() {}
}

impl BrokerSlice for ETradeSlice {
    fn cash_available() {}
    fn positions() {}
    fn order_stock() {}
}