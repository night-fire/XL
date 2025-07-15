use crate::ast::*;

/// A generic visitor trait for walking the AST.
/// Override the `visit_*` methods you are interested in.
pub trait Visitor {
    fn visit_program(&mut self, prog: &Program) {
        for f in &prog.functions {
            self.visit_function(f);
        }
    }

    fn visit_function(&mut self, func: &Function) {
        self.visit_block(&func.body);
    }

    fn visit_block(&mut self, block: &Block) {
        for stmt in &block.stmts {
            self.visit_stmt(stmt);
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { value, .. } => self.visit_expr(value),
            Stmt::Return(e) | Stmt::Expr(e) => self.visit_expr(e),
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        walk_expr(self, expr);
    }
}

/// Helper that walks an expression depth-first calling visitor hooks.
pub fn walk_expr<V: Visitor + ?Sized>(v: &mut V, expr: &Expr) {
    match expr {
        Expr::Binary { left, right, .. } => {
            v.visit_expr(left);
            v.visit_expr(right);
        }
        Expr::Call { args, .. } => {
            for a in args {
                v.visit_expr(a);
            }
        }
        Expr::Match { value, arms } => {
            v.visit_expr(value);
            for (_, e) in arms {
                v.visit_expr(e);
            }
        }
        _ => {}
    }
}