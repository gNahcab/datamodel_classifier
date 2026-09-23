use crate::dasch_project::data_model::data_model_resource::res_property::ResProperty;
use crate::dasch_project::data_model::shared::comments::Comments;
use crate::dasch_project::data_model::shared::labels::Labels;

pub struct DataModelResource {
    pub name: String,
    pub labels: Labels,
    pub res_properties: Vec<ResProperty>,
    pub comments: Comments
}