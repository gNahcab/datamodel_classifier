use crate::dasch_project::data_model::data_model::DataModel;
use crate::dasch_project::descriptions::Descriptions;

pub struct DaschProject {
    pub data_models: Vec<DataModel>,
    pub keywords: Vec<String>,
    pub descriptions: Descriptions,
    pub shortcode: String,
    pub shortname: String,
    pub longname: String,
}
