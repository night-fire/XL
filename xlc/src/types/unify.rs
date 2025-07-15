use super::{ty::{Ty,Tv}, subst::Subst};
use crate::error::XLError;

pub fn unify(a:&Ty, b:&Ty, subst:&mut Subst) -> Result<(), XLError> {
    use Ty::*;
    let a = subst.apply(a);
    let b = subst.apply(b);
    match (a, b) {
        (Var(v), t) | (t, Var(v)) => bind_var(v, &t, subst),
        (Prim(na), Prim(nb)) if na==nb => Ok(()),
        (App(f1,args1), App(f2,args2)) if args1.len()==args2.len() => {
            unify(&f1,&f2,subst)?;
            for (x,y) in args1.iter().zip(args2) { unify(x,y,subst)?; }
            Ok(())
        }
        (Struct(na,pa), Struct(nb,pb)) if na==nb && pa.len()==pb.len() => {
            for (x,y) in pa.iter().zip(pb) { unify(x,y,subst)?; }
            Ok(())
        }
        (Enum(na,pa), Enum(nb,pb)) if na==nb && pa.len()==pb.len() => {
            for (x,y) in pa.iter().zip(pb) { unify(x,y,subst)?; }
            Ok(())
        }
        _ => Err(XLError::SemanticError("type mismatch".into())),
    }
}

fn occurs(tv: Tv, ty:&Ty, subst:&Subst) -> bool {
    use Ty::*;
    match subst.apply(ty) {
        Var(v) => v==tv,
        App(h,args) => occurs(tv, &h, subst) || args.iter().any(|x| occurs(tv,x,subst)),
        Struct(_,fields) | Enum(_,fields) => fields.iter().any(|x| occurs(tv,x,subst)),
        ForAll(_,t)=> occurs(tv,&t,subst),
        _ => false,
    }
}

fn bind_var(tv:Tv, ty:&Ty, subst:&mut Subst) -> Result<(), XLError> {
    if let Ty::Var(v2) = ty { if *v2==tv { return Ok(()); } }
    if occurs(tv, ty, subst) {
        Err(XLError::SemanticError("occurs check failed".into()))
    } else { subst.insert(tv, ty.clone()); Ok(()) }
}