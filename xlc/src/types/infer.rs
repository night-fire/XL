use super::{env::TypeEnv, subst::Subst, ty::{Ty, Tv}, scheme::{Scheme, ftv}};
use crate::ast::{Expr, BinaryOp};
use crate::error::XLError;

#[derive(Debug)]
pub enum TypeError { UnboundVar(String), Mismatch(Ty, Ty) }

pub fn infer_expr(expr: &Expr) -> Result<Ty, XLError> {
    let mut env = TypeEnv::new();
    let mut next = 0u32;
    let (ty, _subst) = infer(expr, &mut env, &mut next)?;
    Ok(ty)
}

fn generalise(env: &TypeEnv, ty: Ty) -> Scheme {
    use std::collections::HashSet;
    let mut free_ty = HashSet::new();
    ftv(&ty, &mut free_ty);
    let mut free_env = HashSet::new();
    for sc in env.map.values() {
        ftv(&sc.ty, &mut free_env);
    }
    free_ty.retain(|tv| !free_env.contains(tv));
    Scheme::new(free_ty.into_iter().collect(), ty)
}

fn instantiate(scheme: &Scheme, next:&mut u32) -> Ty {
    let mut subst = Subst::empty();
    for tv in &scheme.vars {
        subst.insert(*tv, Ty::Var(Tv(*next))); *next +=1;
    }
    subst.apply(&scheme.ty)
}

fn infer(expr: &Expr, env: &mut TypeEnv, next: &mut u32) -> Result<(Ty, Subst), XLError> {
    use Expr::*;
    match expr {
        Int(_) => Ok((Ty::int(), Subst::empty())),
        Bool(_) => Ok((Ty::bool(), Subst::empty())),
        Ident(name) => {
            match env.lookup(name) {
                Some(sc) => {
                    let ty = instantiate(&sc, next);
                    Ok((ty, Subst::empty()))
                }
                None => Err(XLError::SemanticError(format!("unbound identifier {}", name)))
            }
        }
        Binary { op, left, right } => {
            let (t1,s1)=infer(left, env, next)?;
            let (t2,s2)=infer(right, env, next)?;
            // Require operands be Int for arithmetic
            if *op==BinaryOp::Add || *op==BinaryOp::Sub || *op==BinaryOp::Mul || *op==BinaryOp::Div {
                if t1!=Ty::int() || t2!=Ty::int(){ return Err(XLError::SemanticError("type mismatch in binary".into())); }
                Ok((Ty::int(), s1.compose(&s2)))
            } else { Err(XLError::SemanticError("unsupported op".into())) }
        }
        _ => Err(XLError::SemanticError("inference not implemented for expr".into())),
    }
}