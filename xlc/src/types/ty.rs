use super::kind::Kind;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tv(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Var(Tv),
    Prim(&'static str),
    App(Box<Ty>, Vec<Ty>),
    Struct(String, Vec<Ty>),
    Enum(String, Vec<Ty>),
    ForAll(Vec<Tv>, Box<Ty>),
}

impl Ty {
    pub fn int() -> Self { Ty::Prim("Int") }
    pub fn bool() -> Self { Ty::Prim("Bool") }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Var(tv) => write!(f, "t{}", tv.0),
            Ty::Prim(n) => write!(f, "{}", n),
            Ty::App(head, args) => {
                write!(f, "{}<", head)?;
                for (i,a) in args.iter().enumerate() {
                    if i>0 { write!(f, ", ")?; }
                    write!(f, "{}", a)?;
                }
                write!(f, ">")
            }
            Ty::Struct(name, params) => {
                 write!(f, "{}", name)?;
                 if !params.is_empty() {
                     write!(f, "<")?;
                     for (i,a) in params.iter().enumerate() {
                         if i>0 { write!(f, ",")?; }
                         write!(f, "{}", a)?;
                     }
                     write!(f, ">")?;
                 }
                 Ok(())
            }
            Ty::Enum(name, params) => {
                 write!(f, "{}", name)?;
                 if !params.is_empty() {
                     write!(f, "<")?;
                     for (i,a) in params.iter().enumerate() {
                         if i>0 { write!(f, ",")?; }
                         write!(f, "{}", a)?;
                     }
                     write!(f, ">")?;
                 }
                 Ok(())
            }
            Ty::ForAll(vars, t) => {
                write!(f, "∀ ")?;
                for (i,v) in vars.iter().enumerate() {
                    if i>0 { write!(f, ",")?; }
                    write!(f, "t{}", v.0)?;
                }
                write!(f, ". {}", t)
            }
        }
    }
}