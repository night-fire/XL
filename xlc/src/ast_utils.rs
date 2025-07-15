#![allow(dead_code)]
use crate::ast::*;

/// Depth-first traversal of expression tree, calling `f` on every node.
pub fn visit_expr<F: FnMut(&Expr)>(expr: &Expr, f: &mut F) {
    f(expr);
    match expr {
        Expr::Binary { left, right, .. } => {
            visit_expr(left, f);
            visit_expr(right, f);
        }
        Expr::Call { args, .. } => {
            for arg in args {
                visit_expr(arg, f);
            }
        }
        Expr::Match { value, arms } => {
            visit_expr(value, f);
            for (_, e) in arms {
                visit_expr(e, f);
            }
        }
        _ => {}
    }
}

/// Recursively rewrites an expression using the provided closure.
/// If the closure returns `Some(new_expr)` for a node, that node is replaced
/// by `new_expr`. Children of replaced nodes are **not** visited.
pub fn rewrite_expr<F>(expr: Expr, f: &mut F) -> Expr
where
    F: FnMut(Expr) -> Option<Expr>,
{
    if let Some(new) = f(expr.clone()) {
        return new;
    }
    match expr {
        Expr::Binary { op, left, right } => {
            let new_left = Box::new(rewrite_expr(*left, f));
            let new_right = Box::new(rewrite_expr(*right, f));
            Expr::Binary {
                op,
                left: new_left,
                right: new_right,
            }
        }
        Expr::Call { callee, args } => {
            let new_args: Vec<_> = args.into_iter().map(|e| rewrite_expr(e, f)).collect();
            Expr::Call { callee, args: new_args }
        }
        Expr::Match { value, arms } => {
            let new_val = Box::new(rewrite_expr(*value, f));
            let new_arms: Vec<_> = arms
                .into_iter()
                .map(|(p, e)| (p, rewrite_expr(e, f)))
                .collect();
            Expr::Match { value: new_val, arms: new_arms }
        }
        other => other,
    }
}

/// Simple pattern matcher between a `Pattern` and an `Expr`.
/// Currently supports literals, identifiers and wildcard.
pub fn match_pattern(pattern: &Pattern, expr: &Expr) -> bool {
    match (pattern, expr) {
        (Pattern::Wildcard, _) => true,
        (Pattern::Int(a), Expr::Int(b)) => a == b,
        (Pattern::Bool(a), Expr::Bool(b)) => a == b,
        (Pattern::String(a), Expr::String(b)) => a == b,
        (Pattern::Ident(a), Expr::Ident(b)) => a == b,
        _ => false,
    }
}