use thiserror::Error;

#[derive(Error, Debug)]
pub enum XLError {
    #[error("Lexical error at {0}..{1}")]
    LexError(usize, usize),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Semantic error: {0}")]
    SemanticError(String),

    #[error("Code generation error: {0}")]
    CodegenError(String),
}