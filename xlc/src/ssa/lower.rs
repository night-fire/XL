use crate::ssa::{Function as SSAFunc, ValueKind, Terminator};
use crate::ast::*;
use std::collections::HashMap;

pub fn lower_function(ast_fn: &Function) -> SSAFunc {
    let mut ssa = SSAFunc::new(ast_fn.name.clone());
    let entry_id = ssa.entry;
    let entry_block = ssa.blocks.entry(entry_id).or_insert_with(|| super::BasicBlock { id: entry_id, values: Vec::new(), terminator: Terminator::Ret(super::ValueId(0)) });

    let mut env: HashMap<String, super::ValueId> = HashMap::new();

    for stmt in &ast_fn.body.stmts {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let val_id = lower_expr(value, &mut ssa, entry_block, &env);
                env.insert(name.clone(), val_id);
            }
            Stmt::Expr(e) => {
                lower_expr(e, &mut ssa, entry_block, &env);
            }
            Stmt::Return(e) => {
                let v = lower_expr(e, &mut ssa, entry_block, &env);
                entry_block.terminator = Terminator::Ret(v);
            }
        }
    }

    ssa
}

fn lower_expr(expr: &Expr, ssa: &mut SSAFunc, bb: &mut super::BasicBlock, env: &HashMap<String, super::ValueId>) -> super::ValueId {
    use Expr::*;
    match expr {
        Int(v) => {
            let id = ssa.new_value(ValueKind::Const(*v));
            bb.values.push(id);
            id
        }
        Ident(name) => *env.get(name).unwrap_or(&ssa.new_value(ValueKind::Const(0))),
        Binary { op, left, right } => {
            let l = lower_expr(left, ssa, bb, env);
            let r = lower_expr(right, ssa, bb, env);
            let kind = match op {
                BinaryOp::Add => ValueKind::Add(l,r),
                BinaryOp::Sub => ValueKind::Sub(l,r),
                BinaryOp::Mul => ValueKind::Mul(l,r),
                BinaryOp::Div => ValueKind::Div(l,r),
            };
            let id = ssa.new_value(kind);
            bb.values.push(id);
            id
        }
        _ => {
            let id = ssa.new_value(ValueKind::Const(0));
            bb.values.push(id);
            id
        }
    }
}