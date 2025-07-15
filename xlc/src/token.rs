use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    #[token("fn")] Fn,
    #[token("return")] Return,
    #[token("let")] Let,
    #[token("Int")] IntType,
    #[token("Bool")] BoolType,
    #[token("String")] StringType,
    #[token("true")] True,
    #[token("false")] False,
    #[token("match")] Match,
    #[token("module")] ModuleKw,
    #[token("if")] If,
    #[token("rewrite")] RewriteKw,
    #[token("ast")] AstKw,
    #[token("import")] ImportKw,
    #[token("export")] ExportKw,

    // Symbols
    #[token("{")] LBrace,
    #[token("}")] RBrace,
    #[token("(")] LParen,
    #[token(")")] RParen,
    #[token(",")] Comma,
    #[token(":" )] Colon,
    #[token(";")] Semicolon,
    #[token("->")] Arrow,
    #[token("=")] Equal,
    #[token("+")] Plus,
    #[token("-")] Minus,
    #[token("*")] Star,
    #[token("/")] Slash,

    #[token("$")] Dollar,

    #[token("=>")] FatArrow,

    // Pattern / wildcard
    #[token("_")] Underscore,

    // Literals
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    IntLiteral(i64),
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),
    #[regex("\"([^\\\"]|\\.)*\"", |lex| {
        let slice = lex.slice();
        slice[1..slice.len()-1].to_string() // strip quotes
    })]
    StringLiteral(String),

    // Skip whitespace and comments
    #[regex("[ \t\r\n]+", logos::skip)]
    #[regex("//[^\n]*", logos::skip)]
    #[logos(error)]
    Error,
}