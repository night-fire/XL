use std::collections::{HashMap, HashSet};
use crate::ast::*;
use crate::error::XLError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Int,
    Bool,
    Var(u32),
    Fun(Box<Ty>, Box<Ty>),
    Named(String),
    App(Box<Ty>, Vec<Ty>),
}

#[derive(Default)]
struct TIState {
    next_var: u32,
}

impl TIState {
    fn fresh_var(&mut self) -> Ty { let v = self.next_var; self.next_var += 1; Ty::Var(v) }
}

pub fn infer_expr(expr: &Expr) -> Result<Ty, XLError> {
    let mut state = TIState::default();
    let mut env = HashMap::new();
    infer(expr, &mut env, &mut state)
}

fn infer(expr: &Expr, env: &mut HashMap<String, Ty>, st: &mut TIState) -> Result<Ty, XLError> {
    match expr {
        Expr::Int(_) => Ok(Ty::Int),
        Expr::Bool(_) => Ok(Ty::Bool),
        Expr::Ident(name) => env.get(name).cloned().ok_or_else(|| XLError::SemanticError(format!("unbound {:?}", name))),
        Expr::Binary { op: _, left, right } => {
            let l = infer(left, env, st)?;
            let r = infer(right, env, st)?;
            unify(&l, &r)?;
            Ok(l)
        }
        Expr::Call { callee, args } => {
            // very naive: assume callee type is args -> Int
            let arg_types: Vec<_> = args.iter().map(|e| infer(e, env, st)).collect::<Result<_, _>>()?;
            let ret_ty = Ty::Int;
            Ok(ret_ty)
        }
        _ => Err(XLError::SemanticError("type inference: unsupported expr".into())),
    }
}

fn unify(a: &Ty, b: &Ty) -> Result<(), XLError> {
    if a == b { return Ok(()); }
    match (a, b) {
        (Ty::Var(_), _) | (_, Ty::Var(_)) => Ok(()), // TODO occur check
        (Ty::App(fa, args_a), Ty::App(fb, args_b)) if fa == fb && args_a.len() == args_b.len() => {
            for (x, y) in args_a.iter().zip(args_b) { unify(x, y)?; }
            Ok(())
        }
        _ => Err(XLError::SemanticError(format!("cannot unify {:?} and {:?}", a, b)))
    }
}