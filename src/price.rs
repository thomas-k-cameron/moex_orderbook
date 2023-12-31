use rust_decimal::Decimal;

#[derive(Debug, Default, serde::Serialize, Clone, Copy, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "price_type", content = "limit_price")]
pub enum Price {
    Limit(Decimal),
    #[default]
    Market,
}

impl Price {
    pub fn as_limit(&self) -> Option<&Decimal> {
        match &self {
            Self::Limit(d) => Some(d),
            _ => None,
        }
    }
}
