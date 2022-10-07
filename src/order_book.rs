use crate::crate_prelude::*;

#[derive(Default)]
pub struct OrderBook {
    pub id: OrderBookId,
    pub asks: BTreeMap<Decimal, OrderStack>,
    pub bids: BTreeMap<Decimal, OrderStack>,
    pub remaining_market_orders: Vec<OrderLog>,
}

#[derive(Default)]
pub struct OrderStack {
    pub map: HashMap<i64, OrderLog>,
    /// stack is here to track order position
    pub set: Vec<i64>,
}

impl OrderStack {
    pub(crate) fn remove_by_id(&mut self, id: &i64) -> bool {
        if let Some(i) = self.map.remove(&id) {
            if let Some((idx, _)) = self.set.iter().enumerate().find(|(_, item)| **item == i.id) {
                self.set.swap_remove(idx);
                return true
            }
        }
        false
    }
}

//NO      SECCODE BUYSELL TIME    ORDERNO ACTION PRICE VOLUME TRADENO TRADEPRICE
//SYMBOL  SYSTEM  TYPE    MOMENT  ID      ACTION PRICE VOLUME ID_DEAL PRICE_DEAL
