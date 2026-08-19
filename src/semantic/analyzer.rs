use crate::semantic::{
    symbols::{SymbolTable, Symbol},
    types::check_type,
    traits::TraitRegistry,
    casts::{check_as_cast, check_is},
    ownership::{check_ownership, OwnershipOp},
    patterns::validate_match_arms,
    errors::SemanticError,
};
use crate::ir::nodes::*;
use crate::ir::parse_ir::IRRoot;


pub struct Analyzer {
    pub symbols: SymbolTable,
    pub traits: TraitRegistry,
}

impl Analyzer {
    pub fn new(traits: TraitRegistry) -> Self {
        Self {
            symbols: SymbolTable::new(),
            traits,
        }
    }

    pub fn analyze(&mut self, ir: &IRRoot) -> Result<(), SemanticError> {
        self.analyze_module(&ir.module)?;
        self.analyze_types(&ir.types)?;
        self.analyze_functions(&ir.functions)?;
        Ok(())
    }

    fn analyze_module(&mut self, module: &IRModule) -> Result<(), SemanticError> {
        self.symbols.define(&module.name, Symbol::Type { name: module.name.clone() });
        Ok(())
    }

    fn analyze_types(&mut self, types: &Vec<IRTypeDecl>) -> Result<(), SemanticError> {
        for t in types {
            match t {
                IRTypeDecl::Struct(s) => {
                    self.symbols.define(&s.name, Symbol::Type { name: s.name.clone() });
                }
                IRTypeDecl::Enum(e) => {
                    self.symbols.define(&e.name, Symbol::Type { name: e.name.clone() });
                }
                IRTypeDecl::Union(u) => {
                    self.symbols.define(&u.name, Symbol::Type { name: u.name.clone() });
                }
                IRTypeDecl::Trait(tr) => {
                    self.symbols.define(&tr.name, Symbol::Trait { name: tr.name.clone() });
                }
            }
        }
        Ok(())
    }

    fn analyze_functions(&mut self, funcs: &Vec<IRFunction>) -> Result<(), SemanticError> {
        for f in funcs {
            self.symbols.define(
                &f.name,
                Symbol::Func {
                    name: f.name.clone(),
                    params: f.params.iter().map(|p| p.ty.clone()).collect(),
                    ret: f.return_type.clone(),
                },
            );

            self.symbols.enter_scope();
            for p in &f.params {
                self.symbols.define(&p.name, Symbol::Var { name: p.name.clone(), ty: p.ty.clone() });
            }

            for stmt in &f.body {
                self.analyze_statement(stmt)?;
            }

            self.symbols.exit_scope();
        }
        Ok(())
    }

    fn analyze_statement(&mut self, stmt: &IRStatement) -> Result<(), SemanticError> {
        match stmt {
            IRStatement::Let { name, value } => {
                let ty = self.analyze_expr(value)?;
                self.symbols.define(name, Symbol::Var { name: name.clone(), ty });
            }
            IRStatement::Const { name, value } => {
                let ty = self.analyze_expr(value)?;
                self.symbols.define(name, Symbol::Const { name: name.clone(), ty });
            }
            IRStatement::Return(expr) => {
                self.analyze_expr(expr)?;
            }
            IRStatement::Loop(body) => {
                for s in body {
                    self.analyze_statement(s)?;
                }
            }
            IRStatement::If(if_node) => {
                self.analyze_expr(&if_node.condition)?;
                for s in &if_node.then_branch {
                    self.analyze_statement(s)?;
                }
                for s in &if_node.else_branch {
                    self.analyze_statement(s)?;
                }
            }
            IRStatement::Match(m) => {
                self.analyze_expr(&m.scrutinee)?;
                validate_match_arms(&m.arms)?;
            }
            IRStatement::Expr(expr) => {
                self.analyze_expr(expr)?;
            }
        }
        Ok(())
    }

    fn analyze_expr(&mut self, expr: &IRExpr) -> Result<String, SemanticError> {
        match expr {
            IRExpr::Literal(l) => Ok(self.literal_type(l)),
            IRExpr::Var(name) => {
                if let Some(sym) = self.symbols.resolve(name) {
                    match sym {
                        Symbol::Var { ty, .. } => Ok(ty.clone()),
                        Symbol::Const { ty, .. } => Ok(ty.clone()),
                        _ => Err(SemanticError::UndefinedSymbol { name: name.clone() }),
                    }
                } else {
                    Err(SemanticError::UndefinedSymbol { name: name.clone() })
                }
            }
            IRExpr::Binary(b) => {
                let left = self.analyze_expr(&b.left)?;
                let right = self.analyze_expr(&b.right)?;

                match b.kind.as_str() {
                    "AS" => {
                        check_as_cast(&left, &right)?;
                        Ok(right)
                    }
                    "IS" => Ok("bool".to_string()),
                    _ => {
                        check_type(&left, &right)?;
                        Ok(left)
                    }
                }
            }
            IRExpr::Unary(u) => {
                let inner = self.analyze_expr(&u.expr)?;
                match u.kind.as_str() {
                    "MOVE" => {
                        check_ownership(OwnershipOp::Move, &inner)?;
                        Ok(inner)
                    }
                    "CLONE" => {
                        check_ownership(OwnershipOp::Clone, &inner)?;
                        Ok(inner)
                    }
                    "BORROW" => {
                        check_ownership(OwnershipOp::Borrow, &inner)?;
                        Ok(inner)
                    }
                    _ => Ok(inner),
                }
            }
            IRExpr::Call { func, args } => {
                let sym = self.symbols.resolve(func)
                    .ok_or_else(|| SemanticError::UndefinedSymbol { name: func.clone() })?;

                if let Symbol::Func { params, ret, .. } = sym {
                    for (i, arg) in args.iter().enumerate() {
                        let arg_ty = self.analyze_expr(arg)?;
                        check_type(&params[i], &arg_ty)?;
                    }
                    Ok(ret.clone().unwrap_or("none".to_string()))
                } else {
                    Err(SemanticError::UndefinedSymbol { name: func.clone() })
                }
            }
            IRExpr::Pipeline { value, func } => {
                let val_ty = self.analyze_expr(value)?;
                let sym = self.symbols.resolve(func)
                    .ok_or_else(|| SemanticError::UndefinedSymbol { name: func.clone() })?;

                if let Symbol::Func { params, ret, .. } = sym {
                    check_type(&params[0], &val_ty)?;
                    Ok(ret.clone().unwrap_or("none".to_string()))
                } else {
                    Err(SemanticError::UndefinedSymbol { name: func.clone() })
                }
            }
        }
    }

    fn literal_type(&self, lit: &IRLiteral) -> String {
        match lit {
            IRLiteral::Int(_) => "int".to_string(),
            IRLiteral::Float(_) => "float".to_string(),
            IRLiteral::String(_) => "string".to_string(),
            IRLiteral::Bool(_) => "bool".to_string(),
            IRLiteral::None => "none".to_string(),
            IRLiteral::List(_) => "list".to_string(),
        }
    }
}
