use crate::crate_prelude::*;

#[derive(Default)]
pub struct OrderBook {
    pub id: OrderBookId,
    pub asks: BTreeMap<Decimal, OrderStack>,
    pub bids: BTreeMap<Decimal, OrderStack>,
    pub remaining_market_orders: HashMap<i64, DerivativeOrderLog>,
}

#[derive(Default)]
pub struct OrderStack {
    /// {OrderLog::id => OrderLog}
    pub map: HashMap<i64, DerivativeOrderLog>,
    /// stack is here to track order position
    pub set: Vec<i64>,
}

impl OrderStack {
    pub(crate) fn add(&mut self, log: DerivativeOrderLog) {
        self.set.push(log.id);
        self.map.insert(log.id, log);
    }

    pub(crate) fn remove_by_id(&mut self, id: &i64) -> bool {
        if let Some(i) = self.map.remove(&id) {
            if let Ok(idx) = self.set.binary_search_by(|item| item.cmp(&i.id)) {
                self.set.remove(idx);
            } else {
                if cfg!(dev) {
                    unimplemented!()
                };
            }
        }
        false
    }
}

//NO      SECCODE BUYSELL TIME    ORDERNO ACTION PRICE VOLUME TRADENO TRADEPRICE
//SYMBOL  SYSTEM  TYPE    MOMENT  ID      ACTION PRICE VOLUME ID_DEAL PRICE_DEAL
