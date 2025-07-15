use std::collections::HashMap;
use super::ty::{Ty, Tv};

#[derive(Default, Debug, Clone)]
pub struct Subst(pub HashMap<Tv, Ty>);

impl Subst {
    pub fn empty() -> Self { Subst(HashMap::new()) }

    pub fn apply(&self, ty: &Ty) -> Ty {
        match ty {
            Ty::Var(v) => self.0.get(v).cloned().unwrap_or(Ty::Var(*v)),
            Ty::App(h, args) => Ty::App(Box::new(self.apply(h)), args.iter().map(|a| self.apply(a)).collect()),
            Ty::ForAll(vars, t) => {
                let mut inner = self.clone();
                for v in vars { inner.0.remove(v); }
                Ty::ForAll(vars.clone(), Box::new(inner.apply(t)))
            }
            _ => ty.clone(),
        }
    }

    pub fn compose(mut self, other: &Subst) -> Subst {
        for (tv, ty) in other.0.iter() {
            self.0.insert(*tv, self.apply(ty));
        }
        self
    }

    pub fn insert(&mut self, tv: Tv, ty: Ty) {
        self.0.insert(tv, ty);
    }
}