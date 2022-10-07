use crate::TradeLog;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Cancel,
    Add,
    Trade(TradeLog),
}
