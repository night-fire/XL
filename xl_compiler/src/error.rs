use thiserror::Error;

#[derive(Error, Debug)]
pub enum XLError {
    #[error("Parse error: {0}")]
    Parse(String),
}