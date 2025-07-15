use crate::ast::*;

pub trait VisitorMut {
    fn visit_program(&mut self, prog: &mut Program) {
        for f in &mut prog.functions {
            self.visit_function(f);
        }
    }

    fn visit_function(&mut self, func: &mut Function) {
        self.visit_block(&mut func.body);
    }

    fn visit_block(&mut self, block: &mut Block) {
        for stmt in &mut block.stmts {
            self.visit_stmt(stmt);
        }
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) {
        match stmt {
            Stmt::Let { value, .. } => self.visit_expr(value),
            Stmt::Return(e) | Stmt::Expr(e) => self.visit_expr(e),
        }
    }

    fn visit_expr(&mut self, expr: &mut Expr) {
        walk_expr_mut(self, expr);
    }
}

pub fn walk_expr_mut<V: VisitorMut + ?Sized>(v: &mut V, expr: &mut Expr) {
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
        Expr::Rewrite { target, .. } => {
            v.visit_expr(target);
        }
        _ => {}
    }
}