use crate::dasch_project::data_model::data_model_property::data_model_property::DataModelProperty;
use crate::dasch_project::data_model::data_model_resource::data_model_resource::DataModelResource;
use crate::dasch_project::data_model::shared::labels::Labels;

pub struct DataModel {
    pub name: String,
    pub labels: Labels,
    pub comment: String,
    pub properties: Vec<DataModelProperty>,
    pub resources: Vec<DataModelResource>,
}