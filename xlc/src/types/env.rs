use std::collections::HashMap;
use super::scheme::Scheme;
use super::ty::Tv;
use super::subst::Subst;

#[derive(Default, Debug, Clone)]
pub struct TypeEnv {
    pub map: HashMap<String, Scheme>,
}

impl TypeEnv {
    pub fn new() -> Self { TypeEnv::default() }
    pub fn extend(&mut self, name: String, sc: Scheme) { self.map.insert(name, sc); }
    pub fn lookup(&self, name: &str) -> Option<Scheme> { self.map.get(name).cloned() }

    pub fn apply(&self, subst:&Subst) -> TypeEnv {
        let mut new = self.clone();
        for v in new.map.values_mut() {
            v.ty = subst.apply(&v.ty);
        }
        new
    }
}

#[derive(Default, Debug, Clone)]
pub struct KindEnv; // future work