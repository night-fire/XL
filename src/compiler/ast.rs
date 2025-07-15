use super::types::Type;

pub type NodeId = usize;

#[derive(Debug, Clone)]
pub enum Item {
    Function(Function),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_type: Type,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, value: Expr },
    Return(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(String),
    Int(i64),
    Bool(bool),
    Binary { left: Box<Expr>, op: BinOp, right: Box<Expr> },
    Call { func: String, args: Vec<Expr> },
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Eq,
    Ne,
    Lt,
    Gt,
}

pub type Program = Vec<Item>;