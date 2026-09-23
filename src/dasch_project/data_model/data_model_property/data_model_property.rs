use crate::dasch_project::data_model::data_model_property::gui_element::GuiElement;
use crate::dasch_project::data_model::data_model_property::onto_property_value::OntoPropertyValue;
use crate::dasch_project::data_model::data_model_property::property_object::PropertyObject;
use crate::dasch_project::data_model::shared::comments::Comments;
use crate::dasch_project::data_model::shared::labels::Labels;

pub struct DataModelProperty {
    pub name: String,
    pub labels: Labels,
    pub object: PropertyObject,
    pub onto_prop_values: Vec<OntoPropertyValue>,
    pub gui_element: GuiElement,
    pub comments: Comments,
}