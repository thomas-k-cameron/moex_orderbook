use std::str::FromStr;

use chrono::NaiveTime;

use crate::*;

#[derive(
    Debug, Default, serde::Serialize, Clone, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord,
)]
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
            let time_s = iter.next()?.parse::<u64>().ok()?;
            "HHMMSSZZZXXX";

            let hour_mul = 10u64.pow(1 + 3 + 3 + 3);
            let hour = time_s / hour_mul;

            let minutes_mul = 10u64.pow(3 + 3 + 3);
            let minutes = (time_s - hour * hour_mul) / minutes_mul;

            let seconds_mul = 10u64.pow(3 + 3);
            let seconds = (time_s - hour * hour_mul - minutes * minutes_mul) / seconds_mul;

            let micro_mul = 10u64.pow(3);
            let micro = (time_s - hour * hour_mul - minutes * minutes_mul - seconds * seconds_mul)
                / micro_mul;
            NaiveTime::from_hms_micro_opt(hour as u32, minutes as u32, seconds as u32, micro as u32)
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

#[cfg(test)]
mod test {
    use chrono::NaiveTime;

    use crate::OrderLog;

    #[test]
    fn log_parse() {
        let s = [
            "1,ALRS,S,70000000000,1,1,119.79,1600,,",
            "2,ATVI-RM,S,70000000000,2,1,6499,14,,",
            "3,FB-RM,B,70000000000,3,1,17835,1,,",
            "4,TATNP,S,70000000000,4,1,448.6,1,,",
            "5,VIPS-RM,S,70000000000,5,1,750,50,,",
            "6,IRAO,S,70000000000,6,1,3.844,2600,,",
            "7,AFKS,S,70000000000,7,1,20.122,800,,",
            "8,HYDR,B,70000000000,8,1,0.7,50000,,",
            "9,HYDR,B,70000000000,9,1,0.71,25000,,",
        ];
        for i in s {
            if let Some(i) = OrderLog::new(i) {
                assert!(Some(i.time) == NaiveTime::from_hms_opt(7, 0, 0));
            };
        }

        let s = [
            "43309004,MRKP,B,235900000000,20744492,2,0.219,470000,4961540585,0.219",
            "43309005,MRKP,S,235900000000,20744457,2,0.219,470000,4961540585,0.219",
            "43309006,CHMF,B,235900000000,20744496,2,1556,1,4961540586,1556",
            "43309007,CHMF,S,235900000000,20744215,2,1556,1,4961540586,1556",
            "43309008,FXGD,B,235900000000,20743561,2,92.21,6,4961540587,92.21",
            "43309009,FXGD,S,235900000000,20744166,2,92.21,6,4961540587,92.21",
            "43309010,FXRE,B,235900000000,20743125,2,73.84,1,4961540588,73.84",
            "43309011,FXRE,S,235900000000,20744106,2,73.84,1,4961540588,73.84",
            "43309012,GAZP,B,235900000000,20744511,2,320.89,200,4961540589,320.89",
            "43309013,GAZP,S,235900000000,20744133,2,320.89,200,4961540589,320.89",
        ];

        for i in s {
            if let Some(i) = OrderLog::new(i) {
                assert!(Some(i.time) == NaiveTime::from_hms_opt(23, 59, 0));
            };
        }

        if let Some(i) =
            OrderLog::new("41008412,TRUR,S,182808328809,15691327,2,6.02,5,4961481031,6.02")
        {
            assert!(Some(i.time) == NaiveTime::from_hms_micro_opt(18, 28, 83, 2889));
        };

        if let Some(i) = OrderLog::new("41008406,K-RM,B,182808327717,19676999,0,4650,180,,") {
            assert!(Some(i.time) == NaiveTime::from_hms_micro_opt(18, 28, 08, 327717));
        };
    }
}
