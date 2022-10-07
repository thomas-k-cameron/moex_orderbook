pub use crate::crate_prelude::*;

#[derive(Debug)]
pub struct OrderLog {
    /// time/moment
    pub timestamp: NaiveDateTime,
    /// BUYSELL/TYPE
    pub side: Side,
    /// ORDERNO | ID
    pub id: i64,
    pub action: Action,
    pub price: Price,
    pub volume: i64,
    pub name: String,
    pub asset_class: AssetClass,
}

impl OrderLog {
    pub fn new(s: &str) -> Option<Self> {
        let is_derivative = {
            let system_or_seccode = s.split(",").skip(1).next()?;
            let check = if system_or_seccode.len() == 1 {
                let c = system_or_seccode.chars().next()?;
                match c {
                    'F' | 'C' | 'P' => true,
                    _ => false,
                }
            } else {
                return None;
            };
            check
        };

        let timestamp_fmt = "%Y%m%d%H%M%S%f";
        let mut iter = s.split(",");
        // name of the variables matches the `field name` written on the specification
        let order_log = if is_derivative {
            let symbol = iter.next()?;
            let system = AssetClass::new(iter.next()?);
            let r#type = Side::from_str(iter.next()?).ok()?;
            let moment = NaiveDateTime::parse_from_str(iter.next()?, timestamp_fmt).ok()?;
            let id = iter.next()?.parse::<i64>().ok()?;
            let action_byte = iter.next()?;
            let price = {
                let n = iter.next()?;
                let decimal = iter.next()?.parse::<Decimal>().ok()?;
                if n == "0" {
                    Price::Market
                } else {
                    Price::Limit(decimal)
                }
            };
            let volume = iter.next()?.parse::<i64>().ok()?;
            let action = match iter.next()? {
                "" if action_byte == "0" => Action::Cancel,
                "" if action_byte == "1" => Action::Add,
                trade_id => {
                    let price = iter.next()?.parse::<i64>().ok()?;
                    let id = trade_id.parse::<i64>().ok()?;
                    Action::Trade(TradeLog { price, id })
                }
            };

            OrderLog {
                name: symbol.to_string(),
                asset_class: system,
                side: r#type,
                action,
                price,
                volume,
                timestamp: moment,
                id,
            }
        } else {
            let _sequence_number = iter.next()?.parse::<i64>().ok()?;
            let sec_code = iter.next()?.to_string();
            let buy_sell = Side::from_str(iter.next()?).ok()?;
            let time = NaiveDateTime::parse_from_str(iter.next()?, timestamp_fmt).ok()?;
            let order_no = iter.next()?.parse::<i64>().ok()?;
            let action_byte = iter.next()?;
            let price = {
                let n = iter.next()?;
                let decimal = iter.next()?.parse::<Decimal>().ok()?;
                if n == "0" {
                    Price::Market
                } else {
                    Price::Limit(decimal)
                }
            };
            let volume = iter.next()?.parse::<i64>().ok()?;
            let action = match iter.next()? {
                "" if action_byte == "0" => Action::Cancel,
                "" if action_byte == "1" => Action::Add,
                trade_id => {
                    let price = iter.next()?.parse::<i64>().ok()?;
                    let id = trade_id.parse::<i64>().ok()?;
                    Action::Trade(TradeLog { price, id })
                }
            };
            OrderLog {
                name: sec_code,
                id: order_no,
                side: buy_sell,
                timestamp: time,
                action,
                volume,
                price,
                asset_class: AssetClass::EquityOrFX,
            }
        };

        Some(order_log)
    }
}
