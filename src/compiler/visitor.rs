use super::ast::*;

pub trait Visitor {
    fn visit_program(&mut self, prog: &Program) {
        for item in prog {
            self.visit_item(item);
        }
    }

    fn visit_item(&mut self, item: &Item) {
        match item {
            Item::Function(f) => self.visit_function(f),
        }
    }

    fn visit_function(&mut self, fun: &Function) {
        for stmt in &fun.body {
            self.visit_stmt(stmt);
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { value, .. } => self.visit_expr(value),
            Stmt::Return(expr) => self.visit_expr(expr),
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Binary { left, right, .. } => {
                self.visit_expr(left);
                self.visit_expr(right);
            }
            Expr::Call { args, .. } => {
                for a in args {
                    self.visit_expr(a);
                }
            }
            _ => {}
        }
    }
}