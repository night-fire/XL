use logos::Logos;

use crate::token::{Token, TokenKind};

pub fn lex(source: &str) -> Vec<Token> {
    let mut lexer = TokenKind::lexer(source);
    let mut tokens = Vec::new();
    while let Some(kind) = lexer.next() {
        let span = lexer.span();
        let lexeme = lexer.slice().to_string();
        tokens.push(Token {
            kind,
            span,
            lexeme,
        });
    }
    tokens
}