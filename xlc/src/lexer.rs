use crate::token::Token;
use crate::error::XLError;
use logos::Logos;

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub token: T,
    pub span: std::ops::Range<usize>,
}

pub fn lex(source: &str) -> Result<Vec<Spanned<Token>>, XLError> {
    let mut lexer = Token::lexer(source);
    let mut tokens = Vec::new();
    while let Some(tok) = lexer.next() {
        let span = lexer.span();
        match tok {
            Token::Error => {
                return Err(XLError::LexError(span.start, span.end));
            }
            _ => tokens.push(Spanned { token: tok, span }),
        }
    }
    Ok(tokens)
}