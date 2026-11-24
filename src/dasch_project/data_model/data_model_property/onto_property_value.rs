use serde::{Deserialize, Serialize};
use crate::reference_ontologies::onto_name::OntoName;
#[derive(Serialize, Deserialize, Debug)]
pub struct OntoPropertyValue {
    onto_name: OntoName,
    value: String,
}