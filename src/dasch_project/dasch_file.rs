use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::dasch_project::dasch_project::DaschProject;

#[derive(Serialize, Deserialize)]
// Reject unknown fields to prevent silent data loss during deserialization.
#[serde(deny_unknown_fields)]
pub struct DaschFile {
    pub(crate) prefixes: HashMap<String, String>,
   project: DaschProject,
   #[serde(rename="$schema")]
    schema: String
}
