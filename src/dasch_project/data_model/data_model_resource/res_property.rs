use crate::dasch_project::data_model::data_model_resource::cardinality::Cardinality;
use crate::dasch_project::data_model::shared::labels::Labels;

pub struct ResProperty {
    name: String,
    label: Labels,
    cardinality: Cardinality,
}