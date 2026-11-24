use clap::Parser;
#[derive(Parser, Debug)]
pub(crate) struct Cli {
    pub(crate) pattern: String,
    pub(crate) path: std::path::PathBuf,
}