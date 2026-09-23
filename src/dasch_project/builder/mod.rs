use crate::dasch_project::builder::error::DataModelBuilderError;
use crate::dasch_project::data_model::data_model::DataModel;

mod dasch_project_builder;
pub mod error;

pub trait Builder {
    type OutputType;
    fn new(/* ... */) -> Self;
    fn add_data_model(&mut self, data_model: DataModel);
    fn is_complete(&self) -> Result<(), DataModelBuilderError>;

    fn build(self) -> DataModel;
    fn add_shortcode(&mut self, shortcode: String);
    fn add_shortname(&mut self, shortname: String);
}