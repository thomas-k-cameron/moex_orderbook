pub use crate::crate_prelude::*;

#[derive(Hash, PartialEq, Eq, Clone, Default)]
pub struct OrderBookId {
    pub name: Box<str>,
    pub asset_class: AssetClass,
}
