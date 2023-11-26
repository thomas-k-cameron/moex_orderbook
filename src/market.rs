use crate::crate_prelude::*;

pub struct Market {
    pub books: HashMap<OrderBookId, OrderBook>,
}
impl Market {
    pub fn handle(&mut self, log: DerivativeOrderLog) -> Option<DerivativeOrderLog> {
        let id = OrderBookId {
            name: log.name.to_string().into_boxed_str(),
            asset_class: Default::default(),
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
                let check = book.remaining_market_orders.insert(log.id, log); // done
                assert!(check.is_none(), "{check:#?}");
            }
            (Action::Add, Price::Limit(i)) => {
                let stack = half_tree.entry(*i).or_insert_with(OrderStack::default);
                stack.add(log);
                // done
            }
            (Action::Cancel, Price::Market) => {
                if let Some(i) = book.remaining_market_orders.remove(&log.id) {
                    return Some(i);
                } else {
                    unreachable!("{log:#?}");
                };
            }
            (Action::Cancel, Price::Limit(l)) => {
                if let Some(stack) = half_tree.get_mut(l) {
                    assert!(stack.remove_by_id(&log.id), "{log:#?}");
                } else {
                    unreachable!("{log:#?}")
                };
            }
            (Action::Trade(_), Price::Market) => {
                if let Some(i) = book.remaining_market_orders.remove(&log.id) {
                    return Some(i);
                    // done!
                } else {
                    unreachable!("{log:#?}");
                };
            }
            (Action::Trade(_trade), Price::Limit(l)) => {
                if let Some(stack) = half_tree.get_mut(l) {
                    if let Some(to_mut) = stack.map.get_mut(&log.id) {
                        if to_mut.volume == log.volume {
                            assert!(stack.remove_by_id(&log.id), "{log:#?}");
                        } else {
                            to_mut.volume -= log.volume;
                        };
                    } else {
                        unreachable!("{log:#?}");
                    };
                } else {
                    unreachable!("{log:#?}")
                };
            }
        };

        None
    }
}
