use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Cardinality {
    #[serde(rename = "0-n")]
    ZeroToN,
    #[serde(rename = "0-1")]
    ZeroTo1,
    #[serde(rename = "1")]
    One,
    #[serde(rename = "1-n")]
    OneToN,
}