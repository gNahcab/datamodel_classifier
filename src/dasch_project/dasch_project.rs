use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::dasch_project::dasch_list::DaschList;
use crate::dasch_project::data_model::data_model::DataModel;
use crate::dasch_project::error::DaschProjectError;
use crate::dasch_project::language_tag::LanguageTag;
use crate::dasch_project::transient_dasch_project::parse_to_transient_project;
use crate::dasch_project::user::User;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DaschProject {
    pub ontologies: Vec<DataModel>,
    pub keywords: Vec<String>,
    pub descriptions: HashMap<LanguageTag, String>,
    pub enabled_licenses: Vec<String>,
    pub data_copyright_holder: String,
    pub default_permissions: String,
    // groups requires probably not Vec<String>
    pub groups: Vec<String>,
    pub users: Vec<User>,
    pub lists: Vec<DaschList>,
    pub shortcode: String,
    pub shortname: String,
    pub longname: String,
}

impl TryFrom<serde_json::Value> for DaschProject {
    type Error = DaschProjectError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let object = parse_object(value)?;
        parse_to_transient_project(object)?;

        todo!()
    }
}


fn parse_object(value: Value) -> Result<serde_json::map::Map<String, Value>, DaschProjectError> {
    let object = match value {
        Value::Null => {
            return Err(DaschProjectError::ParseError("Expected a datamodel-project, but serde parsed a null-value (probably empty file)".to_string()))
        }
        Value::Bool(bool) => {
            return Err(DaschProjectError::ParseError(format!("Expected a datamodel-project, but serde parsed a bool-value: '{}'", bool)))
        }
        Value::Number(nmbr) => {
            return Err(DaschProjectError::ParseError(format!("Expected a datamodel-project, but serde parsed a number-value: '{}'", nmbr)))
        }
        Value::String(str) => {
            return Err(DaschProjectError::ParseError(format!("Expected a datamodel-project, but serde parsed a string-value: '{}'", str)))
        }
        Value::Array(arr) => {
            return Err(DaschProjectError::ParseError(format!("Expected a datamodel-project, but serde parsed a array-value: '{:?}'", arr)))
        }
        Value::Object(object) => {
            object
        }
    };
    Ok(object)
}