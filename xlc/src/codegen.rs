use crate::ast::*;
use crate::error::XLError;
use anyhow::Result;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::Path;

pub fn emit_llvm_ir(program: &Program, path: &Path) -> Result<(), XLError> {
    let context = Context::create();
    let module = context.create_module("xl_module");
    let builder = context.create_builder();

    let i64_type = context.i64_type();

    // For simplicity, only i64 return type functions for now
    for func in &program.functions {
        let fn_type = i64_type.fn_type(&[], false);
        let function = module.add_function(&func.name, fn_type, None);
        let entry = context.append_basic_block(function, "entry");
        builder.position_at_end(entry);

        // Assuming single return expr
        if let Some(Stmt::Return(expr)) = func.body.stmts.last() {
            let value = codegen_expr(expr, &context, &builder)?;
            builder.build_return(Some(&value));
        } else {
            builder.build_return(Some(&i64_type.const_int(0, false)));
        }
    }

    module.print_to_file(path).map_err(|e| XLError::CodegenError(format!("{}", e)))?;
    Ok(())
}

fn codegen_expr<'ctx>(expr: &Expr, context: &'ctx Context, builder: &inkwell::builder::Builder<'ctx>) -> Result<inkwell::values::IntValue<'ctx>, XLError> {
    let i64_type = context.i64_type();
    match expr {
        Expr::Int(v) => Ok(i64_type.const_int(*v as u64, true)),
        Expr::Binary { op, left, right } => {
            let l = codegen_expr(left, context, builder)?;
            let r = codegen_expr(right, context, builder)?;
            match op {
                BinaryOp::Add => Ok(builder.build_int_add(l, r, "addtmp")),
                BinaryOp::Sub => Ok(builder.build_int_sub(l, r, "subtmp")),
                BinaryOp::Mul => Ok(builder.build_int_mul(l, r, "multmp")),
                BinaryOp::Div => Ok(builder.build_int_signed_div(l, r, "divtmp")),
            }
        }
        Expr::Match { .. } => Err(XLError::CodegenError("match expression codegen not implemented".into())),
        Expr::Rewrite { target, .. } => codegen_expr(target, context, builder),
        _ => Err(XLError::CodegenError("unsupported expression in codegen".into())),
    }
}

pub fn emit_executable(program: &Program, out_path: &Path) -> Result<(), XLError> {
    // Emit IR to temp file and invoke clang to compile
    use std::process::Command;
    let ir_path = out_path.with_extension("ll");
    emit_llvm_ir(program, &ir_path)?;

    // Compile IR to executable with clang if available
    let status = Command::new("clang")
        .arg(&ir_path)
        .arg("-o")
        .arg(out_path)
        .status()
        .map_err(|e| XLError::CodegenError(format!("failed to invoke clang: {}", e)))?;

    if !status.success() {
        return Err(XLError::CodegenError("clang failed".into()));
    }

    Ok(())
}