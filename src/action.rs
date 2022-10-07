use crate::TradeLog;

pub enum Action {
    Cancel,
    Add,
    Trade(TradeLog)
}

