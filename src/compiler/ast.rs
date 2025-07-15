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
    Binary { left: Box<Expr>, op: BinOp, right: Box<Expr> },
    Call { func: String, args: Vec<Expr> },
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Type {
    I32,
    Void,
}

pub type Program = Vec<Item>;