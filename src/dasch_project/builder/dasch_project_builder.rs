use crate::dasch_project::dasch_project::DaschProject;
use crate::dasch_project::builder::Builder;
use crate::dasch_project::builder::error::DataModelBuilderError;
use crate::dasch_project::data_model::data_model::DataModel;

// Purpose of Builder-Pattern: to decouple the process of building the dasch-project-object from the finished dasch-project-object
pub struct DaschProjectBuilder {
    pub data_models: Vec<DataModel>,
    pub shortcode: Option<String>,
    pub shortname: Option<String>,
}
impl Builder for DaschProjectBuilder {

    type OutputType = DaschProject;
    fn new() -> Self {
        Self{
            data_models: vec![],
            shortcode: None,
            shortname: None,
        }
    }

    fn add_data_model(&mut self, data_model: DataModel) {
        todo!()
    }

    fn is_complete(&self) -> Result<(), DataModelBuilderError> {
        todo!()
    }

    fn build(self) -> DataModel {
        todo!()
    }

    fn add_shortcode(&mut self, shortcode: String) {
        todo!()
    }

    fn add_shortname(&mut self, shortname: String) {
        todo!()
    }
}