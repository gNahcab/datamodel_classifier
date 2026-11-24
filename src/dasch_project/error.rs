use thiserror::Error;
use crate::dasch_project::builder::error::DataModelBuilderError;

#[derive(Debug, PartialEq, Error)]
pub enum DaschProjectError {
    #[error("{0}")]
    BuilderError(DataModelBuilderError),
    #[error("{0}")]
    ParseError(String)
}

