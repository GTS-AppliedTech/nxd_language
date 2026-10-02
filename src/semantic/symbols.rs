use std::collections::HashMap;
use crate::ir::nodes::SourceSpan;

#[derive(Clone, Debug)]
pub enum Symbol {
    Var { name: String, ty: String,},
    Const { name: String, ty: String,},
    Func { name: String, params: Vec<String>, ret: Option<String>, is_async: bool, span: Option<SourceSpan>,},
    Type { name: String,},
    Trait { name: String,},
    Process {name: String, params: Vec<String>, ret: Option<String>,},
}

#[derive(Default)] 
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
}

pub struct SymbolTable {
    pub scopes: Vec<Scope>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { scopes: vec![Scope::default()] }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn define(&mut self, name: &str, sym: Symbol) {
        self.scopes.last_mut().unwrap().symbols.insert(name.to_string(), sym);
    }

    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.symbols.get(name) {
                return Some(sym);
            }
        }
        None
    }
}
