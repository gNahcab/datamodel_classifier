use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Hash, Debug, Eq, PartialEq, Clone)]
pub enum LanguageTag{
    #[serde(rename = "en")]
    English,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "rm")]
    Romansh
}