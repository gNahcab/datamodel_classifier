use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::dasch_project::data_model::data_model_resource::res_property::ResProperty;
use crate::dasch_project::language_tag::LanguageTag;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DataModelResource {
    pub name: String,
    #[serde(rename="super")]
    pub _super: Super,
    pub labels: HashMap<LanguageTag, String>,
    pub comments: HashMap<LanguageTag, String>,
    pub cardinalities: Vec<ResProperty>,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
//super can be a string or an array
pub enum Super {
    Singe(String),
    Multiple(Vec<String>),
}