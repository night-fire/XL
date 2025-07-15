#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    I32,
    I64,
    Bool,
    Void,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::I32 => write!(f, "i32"),
            Type::I64 => write!(f, "i64"),
            Type::Bool => write!(f, "bool"),
            Type::Void => write!(f, "void"),
        }
    }
}