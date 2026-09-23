use thiserror::Error;
use crate::dasch_project::builder::error::DataModelBuilderError;

#[derive(Debug, PartialEq, Error)]
pub enum DataProjectError {
    #[error("{0}")]
    BuilderError(DataModelBuilderError),
}
