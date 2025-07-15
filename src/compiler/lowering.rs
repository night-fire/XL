use anyhow::Result;

use super::ast::*;
use super::ir::{BasicBlock, Function as IRFunction, Instruction, Module, ValueId};

pub fn lower(program: &Program) -> Result<Module> {
    let mut module = Module::new();

    for item in program {
        if let Item::Function(fun) = item {
            module.functions.push(lower_function(fun)?);
        }
    }

    Ok(module)
}

fn lower_function(fun: &Function) -> Result<IRFunction> {
    let mut ctx = LowerCtx { next_value: 0 };

    let mut bb = BasicBlock { instrs: Vec::new() };

    // Lower body statements sequentially
    for stmt in &fun.body {
        match stmt {
            Stmt::Return(expr) => {
                let val = lower_expr(expr, &mut ctx, &mut bb)?;
                bb.instrs.push(Instruction::Return { value: Some(val) });
            }
            Stmt::Let { name: _, value: expr } => {
                let _ = lower_expr(expr, &mut ctx, &mut bb)?;
                // Ignore binding for now
            }
        }
    }

    let ir_fun = IRFunction {
        name: fun.name.clone(),
        params: fun.params.iter().map(|p| p.ty.clone()).collect(),
        blocks: vec![bb],
    };
    Ok(ir_fun)
}

struct LowerCtx {
    next_value: ValueId,
}

impl LowerCtx {
    fn fresh(&mut self) -> ValueId {
        let id = self.next_value;
        self.next_value += 1;
        id
    }
}

fn lower_expr(expr: &Expr, ctx: &mut LowerCtx, bb: &mut BasicBlock) -> Result<ValueId> {
    match expr {
        Expr::Int(v) => {
            let dest = ctx.fresh();
            bb.instrs.push(Instruction::ConstInt { dest, value: *v });
            Ok(dest)
        }
        Expr::Binary { left, op, right } => {
            let l = lower_expr(left, ctx, bb)?;
            let r = lower_expr(right, ctx, bb)?;
            let dest = ctx.fresh();
            let instr = match op {
                BinOp::Add => Instruction::Add { dest, lhs: l, rhs: r },
                BinOp::Sub => Instruction::Sub { dest, lhs: l, rhs: r },
                BinOp::Mul => Instruction::Mul { dest, lhs: l, rhs: r },
                BinOp::Div => Instruction::Div { dest, lhs: l, rhs: r },
                _ => anyhow::bail!("Unsupported binary op in lowering"),
            };
            bb.instrs.push(instr);
            Ok(dest)
        }
        _ => anyhow::bail!("Unsupported expression in lowering"),
    }
}