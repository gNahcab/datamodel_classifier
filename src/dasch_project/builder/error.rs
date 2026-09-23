use thiserror::Error;
#[derive(Debug, PartialEq, Error)]
pub enum DataModelBuilderError {
    #[error("{0}")]
    BuilderError(String),
}
