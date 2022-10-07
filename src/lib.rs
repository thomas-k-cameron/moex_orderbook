#![feature(drain_filter)]
use std::collections::{
    BTreeMap,
    HashMap,
    HashSet,
};

use std::str::FromStr;

use chrono::NaiveDateTime;
mod price;
pub use price::Price;
mod derivative_type;
pub use derivative_type::DerivativeType;
mod action;
pub use action::Action;
mod side;
pub use side::Side;
mod asset_class;
pub use asset_class::AssetClass;
mod trade_log;
pub use trade_log::TradeLog;
mod order_log;
pub use order_log::OrderLog;
use rust_decimal::Decimal;

#[derive(Hash, PartialEq, Eq, Clone, Default)]
struct OrderBookId {
    name: String,
    asset_class: AssetClass,
}

#[derive(Default)]
pub struct OrderBook {
    id: OrderBookId,
    asks: BTreeMap<Decimal, OrderStack>,
    bids: BTreeMap<Decimal, OrderStack>,
    remaining_market_orders: Vec<OrderLog>,
}

#[derive(Default)]
pub struct OrderStack {
    map: HashMap<i64, OrderLog>,
    /// stack is here to track order position
    set: HashSet<i64>,
}

struct Market {
    books: HashMap<OrderBookId, OrderBook>,
}
impl Market {
    pub fn handle(&mut self, log: OrderLog) -> Option<OrderLog> {
        let id = OrderBookId {
            name: log.name.to_string(),
            asset_class: log.asset_class,
        };
        let book = if let Some(s) = self.books.get_mut(&id) {
            s
        } else {
            self.books.entry(id.clone()).or_insert_with(|| {
                OrderBook {
                    id,
                    ..Default::default()
                }
            })
        };

        let half_tree = {
            match log.side {
                Side::Buy => &mut book.bids,
                Side::Sell => &mut book.asks,
            }
        };

        match (&log.action, &log.price) {
            (Action::Add, Price::Market) => {
                book.remaining_market_orders.push(log); // done
            }
            (Action::Add, Price::Limit(i)) => {
                let stack = half_tree.entry(*i).or_insert_with(OrderStack::default);
                stack.set.insert(log.id);
                stack.map.insert(log.id, log);
                // done
            }
            (Action::Cancel, Price::Market) => {
                if let Some(i) = book
                    .remaining_market_orders
                    .drain_filter(|i| i.id == log.id)
                    .next()
                {
                    return Some(i);
                } else {
                    unreachable!("{log:#?}");
                };
            }
            (Action::Cancel, Price::Limit(l)) => {
                if let Some(stack) = half_tree.get_mut(l) {
                    if let Some(i) = stack.map.remove(&log.id) {
                        assert!(stack.set.remove(&log.id), "{log:#?}");
                        return Some(i);
                    } else {
                        unreachable!("{log:#?}");
                    }
                } else {
                    unreachable!("{log:#?}")
                };
            }
            (Action::Trade(_), Price::Market) => {
                if let Some(i) = book
                    .remaining_market_orders
                    .drain_filter(|i| i.id == log.id)
                    .next()
                {
                    return Some(i);
                    // done!
                } else {
                    unreachable!("{log:#?}");
                };
            }
            (Action::Trade(_trade), Price::Limit(l)) => {
                if let Some(stack) = half_tree.get_mut(l) {
                    let should_remove_ord = if let Some(to_mut) = stack.map.get_mut(&log.id) {
                        if to_mut.volume == log.volume {
                            true
                        } else {
                            to_mut.volume -= log.volume;
                            false
                        }
                    } else {
                        unreachable!("{log:#?}");
                    };
                    if should_remove_ord {
                        if let Some(_ord) = stack.map.remove(&log.id) {
                            assert!(stack.set.remove(&log.id));
                        } else {
                            unreachable!("{log:#?}")
                        }
                    }
                } else {
                    unreachable!("{log:#?}")
                };
            }
        };

        None
    }
}

//NO      SECCODE BUYSELL TIME    ORDERNO ACTION PRICE VOLUME TRADENO TRADEPRICE
//SYMBOL  SYSTEM  TYPE    MOMENT  ID      ACTION PRICE VOLUME ID_DEAL PRICE_DEAL
