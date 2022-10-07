use rust_decimal::Decimal;

#[derive(Debug)]
pub enum Price {
    Limit(Decimal),
    Market,
}
