use crate::TradeLog;

#[derive(Debug, Default, serde::Serialize, Clone, Copy, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    #[default]
    Cancel,
    Add,
    Trade(TradeLog),
}
