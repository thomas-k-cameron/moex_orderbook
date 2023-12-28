use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize)]
#[serde(tag = "price_type", content = "limit_price")]
pub enum Price {
    Limit(Decimal),
    Market,
}
