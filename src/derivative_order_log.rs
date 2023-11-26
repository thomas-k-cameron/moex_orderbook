pub use crate::crate_prelude::*;

#[derive(Debug)]
pub struct DerivativeOrderLog {
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
    pub derivative_type: DerivativeType,
}

impl DerivativeOrderLog {
    // #SYMBOL,SYSTEM,TYPE,MOMENT,ID,ACTION,PRICE,VOLUME,ID_DEAL,PRICE_DEAL
    pub fn new(s: &str) -> Option<Self> {
        let timestamp_fmt = "%Y%m%d%H%M%S%f";
        let mut iter = s.split(",");
        // name of the variables matches the `field name` written on the specification
        let symbol = iter.next()?;
        let system: DerivativeType = iter.next()?.try_into().ok()?;
        let r#type = Side::from_str(iter.next()?).ok()?;
        let moment = NaiveDateTime::parse_from_str(iter.next()?, timestamp_fmt).ok()?;
        let id = iter.next()?.parse::<i64>().ok()?;
        let action_byte = iter.next()?;
        let price = {
            let decimal = iter.next()?.parse::<Decimal>();
            if let Ok(n) = decimal {
                Price::Limit(n)
            } else {
                Price::Market
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

        Some(DerivativeOrderLog {
            name: symbol.to_string(),
            derivative_type: system,
            side: r#type,
            action,
            price,
            volume,
            timestamp: moment,
            id,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::DerivativeOrderLog;

    #[test]
    fn add() {
        let opts = DerivativeOrderLog::new(
            "Si73750BC2,C,B,20220131185256610,1892947028292403201,1,1.00000,1,,",
        );
        println!("{opts:#?}");
        assert!(opts.is_some());
    }
}
