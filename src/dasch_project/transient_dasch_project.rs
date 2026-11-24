use std::collections::HashMap;
use serde_json::{Map, Value};
use crate::dasch_project::data_model::data_model::DataModel;
use crate::dasch_project::error::DaschProjectError;

pub struct DaschProjectTransient {
    pub ontologies: Vec<DataModel>,
    pub keywords: Vec<String>,
    pub descriptions: HashMap<String, String>,
    pub shortcode: Option<String>,
    pub shortname: Option<String>,
    pub longname: Option<String>,
}
impl DaschProjectTransient {
    fn new() -> Self {
        Self{
            ontologies: vec![],
            keywords: vec![],
            descriptions: Default::default(),
            shortcode: None,
            shortname: None,
            longname: None,
        }
    }

}

pub fn parse_to_transient_project(object: Map<String, Value>) -> Result<DaschProjectTransient, DaschProjectError> {
    let mut dasch_project_transient = DaschProjectTransient::new();
    for (key, value) in object.into_iter() {
        match key.as_str() {
            "$schema" => {}
            "project" => {

            }
            _ => {
                return Err(DaschProjectError::ParseError(format!("Unrecognized key on top-level: {}", key)));
            }
        }
    }

    todo!()
}
