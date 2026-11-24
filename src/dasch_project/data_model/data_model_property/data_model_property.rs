use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::dasch_project::data_model::data_model_property::gui_element::GuiElement;
use crate::dasch_project::language_tag::LanguageTag;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DataModelProperty {
    pub name: String,
    pub labels: HashMap<LanguageTag, String>,
    #[serde(rename = "super")]
    pub super_properties: Vec<String>,
    pub object: String,
    pub gui_element: GuiElement,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gui_attributes: Option<HashMap<String, String>>,
    pub comments: HashMap<String, String>,
}