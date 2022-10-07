use rust_decimal::Decimal;


pub enum Price {
    Limit(Decimal),
    Market
}