use chrono::NaiveTime;

use crate::crate_prelude::*;

pub trait MoexOrderLog: Sized + Send + Clone + Sync {
    type Timestamp: Eq + PartialEq + Ord + PartialOrd + Clone + Sized + Send + Sync + serde::Serialize;
    fn timestamp<'a>(&'a self) -> &'a Self::Timestamp;
    fn ticker<'a>(&'a self) -> &'a str;
    fn side<'a>(&'a self) -> &'a Side;
    fn price<'a>(&'a self) -> &'a Price;
    fn order_no(&self) -> u64;
    fn volume_mut(&mut self) -> &mut i64;
    fn volume(&self) -> i64;
    fn new_from_str(s: &str) -> Option<Self>;
    fn action(&self) -> Action;
    fn seq_num(&self) -> u64;
}

impl MoexOrderLog for DerivativeOrderLog {
    type Timestamp = NaiveDateTime;

    fn seq_num(&self) -> u64 {
        self.id
    }

    fn timestamp<'a>(&'a self) -> &'a Self::Timestamp {
        &self.timestamp
    }

    fn new_from_str(s: &str) -> Option<Self> {
        Self::new(s)
    }

    fn order_no(&self) -> u64 {
        self.id
    }

    fn price<'a>(&'a self) -> &'a Price {
        &self.price
    }

    fn side<'a>(&'a self) -> &'a Side {
        &self.side
    }

    fn ticker<'a>(&'a self) -> &'a str {
        &self.name
    }

    fn volume(&self) -> i64 {
        self.volume
    }

    fn volume_mut(&mut self) -> &mut i64 {
        &mut self.volume
    }

    fn action(&self) -> Action {
        self.action
    }
}

impl MoexOrderLog for OrderLog {
    type Timestamp = NaiveTime;

    fn seq_num(&self) -> u64 {
        self.no
    }

    fn order_no(&self) -> u64 {
        self.orderno
    }

    fn price<'a>(&'a self) -> &'a Price {
        &self.price
    }

    fn side<'a>(&'a self) -> &'a Side {
        &self.buysell
    }

    fn new_from_str(s: &str) -> Option<Self> {
        Self::new(s)
    }

    fn timestamp<'a>(&'a self) -> &'a Self::Timestamp {
        &self.time
    }

    fn ticker<'a>(&'a self) -> &'a str {
        &self.seccode
    }

    fn volume(&self) -> i64 {
        self.volume
    }

    fn volume_mut(&mut self) -> &mut i64 {
        &mut self.volume
    }

    fn action(&self) -> Action {
        self.action
    }
}

#[derive(Default)]
pub struct MoexOrderBook<T>
where
    T: MoexOrderLog,
{
    pub ticker: Box<str>,
    pub asks: Vec<OrderStack<T>>,
    pub bids: Vec<OrderStack<T>>,
}

impl<T> MoexOrderBook<T>
where
    T: MoexOrderLog,
{
    pub fn new(ticker: Box<str>) -> Self {
        Self {
            ticker,
            asks: Vec::with_capacity(1024),
            bids: Vec::with_capacity(1024),
        }
    }

    fn mut_ord_stack(&mut self, side: &Side) -> &mut Vec<OrderStack<T>> {
        let refmut = match side {
            Side::Buy => &mut self.asks,
            Side::Sell => &mut self.bids,
        };
        refmut
    }

    pub fn add(&mut self, log: T) {
        let refmut = self.mut_ord_stack(&log.side());
        let res = refmut.binary_search_by_key(&log.price().as_limit(), |a| Some(&a.price));

        match res {
            Ok(n) => {
                if let Some(item) = refmut.get_mut(n) {
                    item.add(log);
                }
            }
            Err(n) => {
                let limitp = log.price().as_limit().unwrap();
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

    pub fn remove(&mut self, log: &T) -> T {
        let refmut = self.mut_ord_stack(&log.side());
        let res = refmut.binary_search_by_key(&log.price().as_limit(), |a| Some(&a.price));
        match res {
            Ok(idx) => {
                let got = refmut.get_mut(idx).unwrap();
                let item = got.remove_by_id(&log.order_no()).unwrap();
                if got.set.len() == 0 {
                    refmut.remove(idx);
                }
                item
            }
            Err(_) => {
                unreachable!()
            }
        }
    }

    pub fn execute(&mut self, log: &T) {
        let refmut = self.mut_ord_stack(&log.side());
        let res = refmut.binary_search_by_key(&log.price().as_limit(), |a| Some(&a.price));
        match res {
            Ok(idx) => {
                let got = refmut.get_mut(idx).unwrap();
                let (check, id) = if let Some(i) = got.map.get_mut(&log.order_no()) {
                    *i.volume_mut() -= log.volume();
                    (*i.volume_mut() == 0, i.order_no())
                } else {
                    unreachable!()
                };
                if check {
                    got.remove_by_id(&id);
                }
            }
            Err(_) => unreachable!(),
        }
    }
}

#[derive(Default)]
pub struct OrderStack<T> {
    pub price: Decimal,
    /// {OrderLog::id => OrderLog}
    pub map: HashMap<u64, T>,
    /// stack is here to track order position
    pub set: Vec<u64>,
}

impl<T> OrderStack<T>
where
    T: MoexOrderLog,
{
    pub(crate) fn add(&mut self, log: T) {
        self.set.push(log.order_no());
        self.map.insert(log.order_no(), log);
    }

    pub(crate) fn remove_by_id(&mut self, id: &u64) -> Option<T> {
        self.map.remove(id);
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
