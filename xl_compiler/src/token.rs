use logos::Logos;
use std::ops::Range;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    #[token("fn")]
    Fn,
    #[token("let")]
    Let,
    #[token("return")]
    Return,

    // Symbols
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,
    #[token(",")]
    Comma,
    #[token(";")]
    Semi,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("=")]
    Equal,

    // Identifiers and literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,
    #[regex(r"[0-9]+")]
    Int,

    // Whitespace and errors (skipped)
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    #[error]
    Error,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Range<usize>,
    pub lexeme: String,
}