use crate::ast::*;
use crate::error::XLError;
use std::collections::HashMap;

pub fn analyze(program: &Program) -> Result<(), XLError> {
    let mut ctx = Context::new();
    // Register function signatures first
    for func in &program.functions {
        if ctx.functions.contains_key(&func.name) {
            return Err(XLError::SemanticError(format!(
                "duplicate function {}",
                func.name
            )));
        }
        ctx.functions.insert(func.name.clone(), func.return_type.clone());
    }

    for func in &program.functions {
        analyze_function(func, &ctx)?;
    }

    Ok(())
}

struct Context {
    functions: HashMap<String, Type>,
}

impl Context {
    fn new() -> Self {
        Context {
            functions: HashMap::new(),
        }
    }
}

fn analyze_function(func: &Function, ctx: &Context) -> Result<(), XLError> {
    for stmt in &func.body.stmts {
        match stmt {
            Stmt::Return(expr) => {
                let ty = infer_expr_type(expr, ctx)?;
                if ty != func.return_type {
                    return Err(XLError::SemanticError(format!(
                        "return type mismatch in function {}: expected {:?}, found {:?}",
                        func.name, func.return_type, ty
                    )));
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn infer_expr_type(expr: &Expr, ctx: &Context) -> Result<Type, XLError> {
    match expr {
        Expr::Int(_) => Ok(Type::Int),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::String(_) => Ok(Type::String),
        Expr::Ident(name) => ctx
            .functions
            .get(name)
            .cloned()
            .ok_or_else(|| XLError::SemanticError(format!("unknown identifier {}", name))),
        Expr::Binary { op: _, left, right } => {
            let lty = infer_expr_type(left, ctx)?;
            let rty = infer_expr_type(right, ctx)?;
            if lty == rty {
                Ok(lty)
            } else {
                Err(XLError::SemanticError(format!(
                    "type mismatch in binary expression: {:?} vs {:?}",
                    lty, rty
                )))
            }
        }
        Expr::Call { callee, args: _ } => ctx
            .functions
            .get(callee)
            .cloned()
            .ok_or_else(|| XLError::SemanticError(format!("unknown function {}", callee))),
    }
}