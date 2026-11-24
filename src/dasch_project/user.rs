use serde::{Deserialize, Serialize};
use crate::dasch_project::language_tag::LanguageTag;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub username: String,
    pub email: String,
    pub givenName: String,
    pub familyName: String,
    pub password: String,
    pub lang: LanguageTag,
    pub groups: Vec<String>,
    pub projects: Vec<String>,
    pub status: bool,
}