use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::dasch_project::language_tag::LanguageTag;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DaschList {
    pub name: String,
    pub labels: HashMap<LanguageTag, String>,
    pub comments: HashMap<LanguageTag, String>,
    pub nodes: Vec<ListNode>
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListNode {
    pub name: String,
    pub labels: HashMap<LanguageTag, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<ListNode>>
}