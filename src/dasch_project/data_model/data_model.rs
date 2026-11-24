use serde::{Deserialize, Serialize};
use crate::dasch_project::data_model::data_model_property::data_model_property::DataModelProperty;
use crate::dasch_project::data_model::data_model_resource::data_model_resource::DataModelResource;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DataModel {
    pub name: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub properties: Vec<DataModelProperty>,
    pub resources: Vec<DataModelResource>,
}