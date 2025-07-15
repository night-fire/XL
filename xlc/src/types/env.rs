use std::collections::HashMap;
use super::scheme::Scheme;
use super::ty::Tv;

#[derive(Default, Debug, Clone)]
pub struct TypeEnv {
    pub map: HashMap<String, Scheme>,
}

impl TypeEnv {
    pub fn new() -> Self { TypeEnv::default() }
    pub fn extend(&mut self, name: String, sc: Scheme) { self.map.insert(name, sc); }
    pub fn lookup(&self, name: &str) -> Option<Scheme> { self.map.get(name).cloned() }
}

#[derive(Default, Debug, Clone)]
pub struct KindEnv; // future work