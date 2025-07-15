use crate::ssa::{Function as SSAFunc, ValueKind, Terminator};
use crate::ast::*;

pub fn lower_function(ast_fn: &Function) -> SSAFunc {
    let mut ssa = SSAFunc::new(ast_fn.name.clone());
    let entry_id = ssa.entry;
    let entry_block = ssa.blocks.entry(entry_id).or_insert_with(|| super::BasicBlock { id: entry_id, values: Vec::new(), terminator: Terminator::Ret(super::ValueId(0)) });

    if let Some(Stmt::Return(expr)) = ast_fn.body.stmts.last() {
        let val = lower_expr(expr, &mut ssa, entry_block);
        entry_block.terminator = Terminator::Ret(val);
    }

    ssa
}

fn lower_expr(expr: &Expr, ssa: &mut SSAFunc, bb: &mut super::BasicBlock) -> super::ValueId {
    use Expr::*;
    match expr {
        Int(v) => {
            let id = ssa.new_value(ValueKind::Const(*v));
            bb.values.push(id);
            id
        }
        Binary { op, left, right } => {
            let l = lower_expr(left, ssa, bb);
            let r = lower_expr(right, ssa, bb);
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
            // unsupported -> const 0
            let id = ssa.new_value(ValueKind::Const(0));
            bb.values.push(id);
            id
        }
    }
}