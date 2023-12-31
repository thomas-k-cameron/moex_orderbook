use crate::crate_prelude::*;

#[derive(Default)]
pub struct OrderBook {
    pub id: OrderBookId,
    pub asks: Vec<OrderStack>,
    pub bids: Vec<OrderStack>,
    pub market_orders: Vec<DerivativeOrderLog>,
}

impl OrderBook {
    fn mut_ord_stack(&mut self, side: &Side) -> &mut Vec<OrderStack> {
        let refmut = match side {
            Side::Buy => &mut self.asks,
            Side::Sell => &mut self.bids,
        };
        refmut
    }

    pub fn add(&mut self, log: DerivativeOrderLog) {
        let refmut = self.mut_ord_stack(&log.side);
        let res = refmut.binary_search_by_key(&log.price.as_limit(), |a| Some(&a.price));

        match res {
            Ok(n) => {
                if let Some(item) = refmut.get_mut(n) {
                    item.add(log);
                }
            }
            Err(n) => {
                let limitp = log.price.as_limit().unwrap();
                let mut stack = OrderStack {
                    price: *limitp,
                    map: HashMap::with_capacity(1024),
                    set: Vec::with_capacity(1024),
                };
                stack.add(log);
                refmut.insert(n, stack);
            }
        }
    }

    pub fn remove(&mut self, log: DerivativeOrderLog) -> DerivativeOrderLog {
        let refmut = self.mut_ord_stack(&log.side);
        let res = refmut.binary_search_by_key(&log.price.as_limit(), |a| Some(&a.price));
        match res {
            Ok(idx) => {
                let got = refmut.get_mut(idx).unwrap();
                got.remove_by_id(&log.id).unwrap()
            }
            Err(_) => {
                unreachable!()
            }
        }
    }

    pub fn execute(&mut self, log: DerivativeOrderLog) {
        let refmut = self.mut_ord_stack(&log.side);
        let res = refmut.binary_search_by_key(&log.price.as_limit(), |a| Some(&a.price));
        match res {
            Ok(idx) => {
                let got = refmut.get_mut(idx).unwrap();
                if let Some(i) = got.map.get_mut(&log.id) {
                    i.volume -= log.volume;
                    return;
                }
            }
            Err(_) => {}
        }
        unreachable!()
    }
}

#[derive(Default)]
pub struct OrderStack {
    pub price: Decimal,
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

    pub(crate) fn remove_by_id(&mut self, id: &i64) -> Option<DerivativeOrderLog> {
        if let Ok(idx) = self.set.binary_search_by(|item| item.cmp(&id)) {
            self.set.remove(idx);
            
        } else {
            if cfg!(dev) {
                unimplemented!()
            };
        }

        self.map.remove(&id)
    }
}

//NO      SECCODE BUYSELL TIME    ORDERNO ACTION PRICE VOLUME TRADENO TRADEPRICE
//SYMBOL  SYSTEM  TYPE    MOMENT  ID      ACTION PRICE VOLUME ID_DEAL PRICE_DEAL
