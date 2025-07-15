use anyhow::{Context, Result};

use super::ast::*;
use super::types::Type;

pub fn check_program(prog: &Program) -> Result<()> {
    for item in prog {
        if let Item::Function(fun) = item {
            check_function(fun).with_context(|| format!("In function {}", fun.name))?;
        }
    }
    Ok(())
}

fn check_function(fun: &Function) -> Result<()> {
    for stmt in &fun.body {
        match stmt {
            Stmt::Return(expr) => {
                let ty = infer_expr(expr)?;
                if ty != fun.ret_type {
                    anyhow::bail!(
                        "Return type mismatch: expected {}, got {}",
                        fun.ret_type,
                        ty
                    );
                }
            }
            Stmt::Let { name: _, value: expr } => {
                // For MVP just ensure expression is inferrable
                let _ = infer_expr(expr)?;
            }
        }
    }
    Ok(())
}

fn infer_expr(expr: &Expr) -> Result<Type> {
    match expr {
        Expr::Int(_) => Ok(Type::I32),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::Binary { left, op, right } => {
            let l = infer_expr(left)?;
            let r = infer_expr(right)?;
            if l != r {
                anyhow::bail!("Type mismatch in binary expression: {} vs {}", l, r);
            }
            match op {
                BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                    if l != Type::I32 {
                        anyhow::bail!("Operator requires I32 operands")
                    }
                    Ok(Type::I32)
                }
                BinOp::And | BinOp::Or => {
                    if l != Type::Bool {
                        anyhow::bail!("Logical operator requires bool operands")
                    }
                    Ok(Type::Bool)
                }
                BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt => {
                    // comparison returns bool, operands must be comparable
                    Ok(Type::Bool)
                }
            }
        }
        Expr::Ident(_) => {
            // Variable types not implemented yet
            Ok(Type::I32)
        }
        Expr::Call { func: _, args: _ } => {
            // For MVP assume call returns I32
            Ok(Type::I32)
        }
    }
}