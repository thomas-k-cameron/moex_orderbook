use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[serde(tag = "price_type", content = "limit_price")]
pub enum Price {
    Limit(Decimal),
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
