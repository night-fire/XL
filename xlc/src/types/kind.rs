use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Kind {
    Star,
    Arrow(Box<Kind>, Box<Kind>),
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Star => write!(f, "*"),
            Kind::Arrow(a, b) => write!(f, "({} -> {})", a, b),
        }
    }
}