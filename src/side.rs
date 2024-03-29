use std::str::FromStr;

#[derive(
    Debug,
    Default,
    serde::Serialize,
    Clone,
    Copy,
    serde::Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum Side {
    #[default]
    Buy,
    Sell,
}
impl FromStr for Side {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "B" => Side::Buy,
            "S" => Side::Sell,
            _ => return Err(()),
        })
    }
}
