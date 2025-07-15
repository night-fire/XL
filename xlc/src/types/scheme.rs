use super::ty::{Ty, Tv};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Scheme {
    pub vars: Vec<Tv>,
    pub ty: Ty,
}

impl Scheme {
    pub fn new(vars: Vec<Tv>, ty: Ty) -> Self { Scheme { vars, ty } }
}

/// Collect free type variables in a type
pub fn ftv(ty: &Ty, acc: &mut HashSet<Tv>) {
    use Ty::*;
    match ty {
        Var(v) => { acc.insert(*v); },
        App(h, args) => { ftv(h, acc); for a in args { ftv(a, acc);} }
        Struct(_, params) | Enum(_, params) => { for a in params { ftv(a, acc);} }
        ForAll(_, t) => ftv(t, acc),
        Prim(_) => {}
    }
}