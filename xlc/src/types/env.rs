use std::collections::HashMap;
use super::ty::{Ty, Tv};

#[derive(Default, Debug, Clone)]
pub struct TypeEnv {
    map: HashMap<String, Ty>,
}

impl TypeEnv {
    pub fn new() -> Self { TypeEnv::default() }
    pub fn extend(&mut self, name: String, ty: Ty) { self.map.insert(name, ty); }
    pub fn lookup(&self, name: &str) -> Option<Ty> { self.map.get(name).cloned() }
}

#[derive(Default, Debug, Clone)]
pub struct KindEnv; // future work