use crate::lexer::Spanned;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>, // top-level
    pub modules: Vec<Module>,
    pub ast_decls: Vec<AstDecl>,
    pub imports: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Type,
    pub body: Block,
    pub is_export: bool,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        ty: Option<Type>,
        value: Expr,
    },
    Return(Expr),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Bool(bool),
    String(String),
    Ident(String),
    Node {
        name: String,
        args: Vec<Pattern>,
    },
    Match {
        value: Box<Expr>,
        arms: Vec<(Pattern, Expr)>,
    },
    Rewrite {
        rules: Vec<RewriteRule>,
        target: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        callee: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Bool,
    String,
    Void,
}

impl Type {
    pub fn from_keyword(tok: &Token) -> Option<Self> {
        match tok {
            Token::IntType => Some(Type::Int),
            Token::BoolType => Some(Type::Bool),
            Token::StringType => Some(Type::String),
            _ => None,
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int)
    }

    pub fn unify(a: &Type, b: &Type) -> Option<Type> {
        if a == b { Some(a.clone()) } else { None }
    }
}

#[derive(Debug, Clone)]
pub enum Pattern {
    /// `_` matches anything
    Wildcard,
    /// `$name` — захват произвольного выражения в переменную
    Capture(String),
    Int(i64),
    Bool(bool),
    String(String),
    /// Точное совпадение идентификатора (например, `x`)
    Ident(String),

    /// Двоичное выражение с необязательным указанием оператора (None = любой)
    Binary {
        op: Option<BinaryOp>,
        left: Box<Pattern>,
        right: Box<Pattern>,
    },

    /// Вызов функции. `callee = None` соответствует любому идентификатору
    Call {
        callee: Option<String>,
        args: Vec<Pattern>,
    },
}

#[derive(Debug, Clone)]
pub struct RewriteRule {
    pub pattern: Pattern,
    pub guard: Option<Expr>, // evaluated with captures
    pub replacement: Expr,
}

#[derive(Debug, Clone)]
pub struct AstField {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone)]
pub struct AstDecl {
    pub name: String,
    pub fields: Vec<AstField>,
}

pub type SpannedToken = Spanned<Token>;