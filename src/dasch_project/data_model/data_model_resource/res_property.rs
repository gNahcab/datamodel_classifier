use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::dasch_project::data_model::data_model_resource::cardinality::Cardinality;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ResProperty {
    propname: String,
    cardinality: Cardinality,
    gui_order: usize,
}