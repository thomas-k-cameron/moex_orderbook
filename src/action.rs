use crate::TradeLog;

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum Action {
    Cancel,
    Add,
    Trade,
}
