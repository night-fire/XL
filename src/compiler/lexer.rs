use anyhow::Result;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum TokenKind {
    #[token("function")]
    Function,
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Int(i64),
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,
    #[token(";")]
    Semi,
    #[error]
    #[regex("[ \t\n\r]+", logos::skip)]
    Error,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub span: std::ops::Range<usize>,
}

pub fn lex(source: &str) -> Result<Vec<Token>> {
    let mut lexer = TokenKind::lexer(source);
    let mut tokens = Vec::new();
    while let Some(kind) = lexer.next() {
        let span = lexer.span();
        let text = source[span.clone()].to_string();
        let token = Token { kind: kind.clone(), text, span };
        if matches!(kind, TokenKind::Error) {
            anyhow::bail!("Lexing error at position {}", span.start);
        }
        tokens.push(token);
    }
    Ok(tokens)
}