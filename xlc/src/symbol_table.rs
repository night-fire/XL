use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolTable<T> {
    scopes: Vec<HashMap<String, T>>,
}

impl<T: Clone> SymbolTable<T> {
    pub fn new() -> Self { Self { scopes: vec![HashMap::new()] } }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn insert(&mut self, name: String, value: T) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    pub fn get(&self, name: &str) -> Option<T> {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v.clone());
            }
        }
        None
    }
}