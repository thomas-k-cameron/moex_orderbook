use std::str::FromStr;

use chrono::NaiveDateTime;

use crate::*;

pub struct EquityOrderLog {
    pub no: u64,
    pub seccode: Box<str>,
    pub buysell: Side,
    pub time: NaiveDateTime,
    pub orderno: u64,
    pub action: Action,
    pub price: Price,
    pub volume: i64,
}

impl EquityOrderLog {
    pub fn new(s: &str) -> Option<Self> {
        let mut iter = s.split(",");
        let timestamp_fmt = "%Y%m%d%H%M%S%f";

        let no = iter.next()?.parse::<u64>().ok()?;
        let seccode = iter.next()?.to_string().into_boxed_str();
        let buysell = Side::from_str(iter.next()?).ok()?;
        let time = NaiveDateTime::parse_from_str(iter.next()?, timestamp_fmt).ok()?;
        let orderno = iter.next()?.parse::<u64>().ok()?;
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
        Some(Self {
            no,
            seccode,
            buysell,
            time,
            orderno,
            action,
            price,
            volume,
        })
    }
}
