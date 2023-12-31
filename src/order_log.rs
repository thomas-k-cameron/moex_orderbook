use std::str::FromStr;

use chrono::NaiveTime;

use crate::*;

pub struct OrderLog {
    pub no: u64,
    pub seccode: Box<str>,
    pub buysell: Side,
    pub time: NaiveTime,
    pub orderno: u64,
    pub action: Action,
    pub price: Price,
    pub volume: i64,
}

impl OrderLog {
    pub fn new(s: &str) -> Option<Self> {
        let mut iter = s.split(",");

        let no = iter.next()?.parse::<u64>().ok()?;
        let seccode = iter.next()?.to_string().into_boxed_str();
        let buysell = Side::from_str(iter.next()?).ok()?;
        let time = {
            let time_s = iter.next()?;
            let hour = time_s[..2].parse().ok()?;
            let min = time_s[2..4].parse().ok()?;
            let sec = time_s[4..6].parse().ok()?;
            let micro = time_s[6..12].parse().ok()?;
            NaiveTime::from_hms_micro_opt(hour, min, sec, micro)
                .ok_or(())
                .ok()?
        };
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
