use crate::compiler::ast::*;

pub fn generate_rust(program: &Program) -> String {
    let mut out = String::new();
    out.push_str("// Auto-generated Rust code from XL compiler\n");
    for item in program {
        match item {
            Item::Function(fun) => gen_function(fun, &mut out),
        }
    }
    // Ensure main exists; else create empty
    if !program.iter().any(|i| matches!(i, Item::Function(f) if f.name == "main")) {
        out.push_str("fn main() {}\n");
    }
    out
}

fn gen_function(fun: &Function, out: &mut String) {
    out.push_str("fn ");
    out.push_str(&fun.name);
    out.push('(');
    for (i, p) in fun.params.iter().enumerate() {
        if i > 0 { out.push(','); }
        out.push_str(&p.name);
        out.push_str(": i32");
    }
    out.push(')');
    match fun.ret_type {
        Type::I32 => out.push_str(" -> i32"),
        Type::Void => {},
    }
    out.push_str(" {\n");
    for stmt in &fun.body {
        gen_stmt(stmt, out);
    }
    out.push_str("}\n\n");
}

fn gen_stmt(stmt: &Stmt, out: &mut String) {
    match stmt {
        Stmt::Return(expr) => {
            out.push_str("    return ");
            gen_expr(expr, out);
            out.push_str(";\n");
        }
        Stmt::Let { name, value } => {
            out.push_str("    let ");
            out.push_str(name);
            out.push_str(" = ");
            gen_expr(value, out);
            out.push_str(";\n");
        }
    }
}

fn gen_expr(expr: &Expr, out: &mut String) {
    match expr {
        Expr::Ident(name) => out.push_str(name),
        Expr::Int(v) => out.push_str(&v.to_string()),
        Expr::Binary { left, op, right } => {
            gen_expr(left, out);
            out.push(' ');
            out.push_str(match op {
                BinOp::Add => "+",
                BinOp::Sub => "-",
                BinOp::Mul => "*",
                BinOp::Div => "/",
            });
            out.push(' ');
            gen_expr(right, out);
        }
        Expr::Call { func, args } => {
            out.push_str(func);
            out.push('(');
            for (i, arg) in args.iter().enumerate() {
                if i > 0 { out.push(','); }
                gen_expr(arg, out);
            }
            out.push(')');
        }
    }
}