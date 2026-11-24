use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelsWrapperError {
    #[error("Qwen model error: {0}")]
    QwenModelError(#[from] reqwest::Error)
}
