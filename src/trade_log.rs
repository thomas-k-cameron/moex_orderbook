#[derive(Debug, Default, serde::Serialize, Clone, Copy, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TradeLog {
    /// price the order was executed at
    /// TRADEPRICE/PRICE_DEAL
    pub price: i64,
    /// ID_DEAL/TRADENO
    pub id: i64,
}
