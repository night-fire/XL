use super::{env::TypeEnv, subst::Subst, ty::{Ty, Tv}, scheme::{Scheme, ftv}, unify::unify};
use crate::ast::{Expr, BinaryOp, Block, Stmt};
use crate::error::XLError;

#[derive(Debug)]
pub enum TypeError { UnboundVar(String), Mismatch(Ty, Ty) }

pub fn infer_expr(expr: &Expr) -> Result<Ty, XLError> {
    let mut env = TypeEnv::new();
    let mut next = 0u32;
    let (ty, _subst) = infer(expr, &mut env, &mut next)?;
    Ok(ty)
}

pub fn infer_block(block:&Block) -> Result<Ty, XLError> {
    let mut env = TypeEnv::new();
    let mut next=0u32;
    let mut last_ty = Ty::int();
    for stmt in &block.stmts {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let (ty,s) = infer(value,&mut env,&mut next)?;
                let sc = generalise(&env.apply(&s), ty.clone());
                env.extend(name.clone(), sc);
            }
            Stmt::Expr(e) | Stmt::Return(e) => {
                let (ty,_) = infer(e,&mut env,&mut next)?;
                last_ty = ty;
            }
        }
    }
    Ok(last_ty)
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
                let mut s = s1.compose(&s2);
                unify(&t1, &Ty::int(), &mut s)?;
                unify(&t2, &Ty::int(), &mut s)?;
                Ok((Ty::int(), s))
            } else { Err(XLError::SemanticError("unsupported op".into())) }
        }
        _ => Err(XLError::SemanticError("inference not implemented for expr".into())),
    }
}