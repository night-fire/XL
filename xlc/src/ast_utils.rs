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

use std::collections::HashMap;

pub type Bindings<'a> = HashMap<String, &'a Expr>;

/// Match pattern against expression, collecting captures into `bindings`.
pub fn match_pattern<'a>(pattern: &'a Pattern, expr: &'a Expr, bindings: &mut Bindings<'a>) -> bool {
    match (pattern, expr) {
        (Pattern::Wildcard, _) => true,
        (Pattern::Capture(name), e) => {
            // Ensure if already bound, it's the same expression
            if let Some(prev) = bindings.get(name) {
                std::ptr::eq(*prev as *const _, e as *const _)
            } else {
                bindings.insert(name.clone(), e);
                true
            }
        }
        (Pattern::Int(a), Expr::Int(b)) => a == b,
        (Pattern::Bool(a), Expr::Bool(b)) => a == b,
        (Pattern::String(a), Expr::String(b)) => a == b,
        (Pattern::Ident(a), Expr::Ident(b)) => a == b,
        (
            Pattern::Binary { op: popt, left: pl, right: pr },
            Expr::Binary { op: eop, left: el, right: er },
        ) => {
            (popt.map_or(true, |o| o == *eop))
                && match_pattern(pl, el, bindings)
                && match_pattern(pr, er, bindings)
        }
        (
            Pattern::Call { callee: pc, args: pargs },
            Expr::Call { callee: ec, args: eargs },
        ) => {
            let callee_ok = pc.as_ref().map_or(true, |name| name == ec);
            callee_ok && pargs.len() == eargs.len() && pargs.iter().zip(eargs).all(|(p, e)| match_pattern(p, e, bindings))
        }
        (
            Pattern::Node { name: pn, args: pargs },
            expr @ _
        ) => {
            // naive: match Expr::Ident of node? For now we can't destruct Expr nodes generically. We'll compare Debug.
            if let Expr::Call { callee, args: eargs } = expr {
                let callee_match = pn == callee;
                callee_match && pargs.len() == eargs.len() && pargs.iter().zip(eargs).all(|(p,e)| match_pattern(p,e,bindings))
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Compatibility wrapper ignoring captures
pub fn match_pattern_simple(pattern: &Pattern, expr: &Expr) -> bool {
    let mut b = Bindings::new();
    match_pattern(pattern, expr, &mut b)
}

/// Apply rewrite rules with guards and capture substitution (new API)
pub fn apply_rewrite_rules2(expr: Expr, rules: &[crate::ast::RewriteRule]) -> Expr {
    use crate::ast::RewriteRule;
    // Attempt to match each rule in order
    for RewriteRule { pattern, guard, replacement } in rules {
        let mut binds = Bindings::new();
        if match_pattern(pattern, &expr, &mut binds) {
            // Check guard if any
            let guard_ok = match guard {
                None => true,
                Some(Expr::Bool(b)) => *b,
                _ => false, // Only constant bool for now
            };
            if !guard_ok { continue; }

            // Substitute captures in replacement
            return substitute_captures(replacement.clone(), &binds);
        }
    }
    // Recurse
    match expr {
        Expr::Binary { op, left, right } => Expr::Binary {
            op,
            left: Box::new(apply_rewrite_rules2(*left, rules)),
            right: Box::new(apply_rewrite_rules2(*right, rules)),
        },
        Expr::Call { callee, args } => Expr::Call {
            callee,
            args: args.into_iter().map(|e| apply_rewrite_rules2(e, rules)).collect(),
        },
        Expr::Match { value, arms } => Expr::Match {
            value: Box::new(apply_rewrite_rules2(*value, rules)),
            arms: arms.into_iter().map(|(p, e)| (p, apply_rewrite_rules2(e, rules))).collect(),
        },
        other => other,
    }
}

fn substitute_captures(expr: Expr, binds: &Bindings) -> Expr {
    match expr {
        Expr::Ident(name) => {
            if let Some(e) = binds.get(&name) {
                (*e).clone()
            } else {
                Expr::Ident(name)
            }
        }
        Expr::Binary { op, left, right } => Expr::Binary {
            op,
            left: Box::new(substitute_captures(*left, binds)),
            right: Box::new(substitute_captures(*right, binds)),
        },
        Expr::Call { callee, args } => Expr::Call {
            callee,
            args: args.into_iter().map(|e| substitute_captures(e, binds)).collect(),
        },
        other => other,
    }
}

pub fn apply_rewrite_rules(expr: Expr, rules: &[crate::ast::RewriteRule]) -> Expr {
    apply_rewrite_rules2(expr, rules)
}

/// Fold expression tree aggregating value `A` using provided closure.
pub fn fold_expr<A, F>(expr: &Expr, init: A, mut f: F) -> A
where
    F: FnMut(A, &Expr) -> A,
    A: Clone,
{
    let mut acc = f(init, expr);
    match expr {
        Expr::Binary { left, right, .. } => {
            acc = fold_expr(left, acc, &mut f);
            acc = fold_expr(right, acc, &mut f);
        }
        Expr::Call { args, .. } => {
            for a in args {
                acc = fold_expr(a, acc, &mut f);
            }
        }
        Expr::Match { value, arms } => {
            acc = fold_expr(value, acc, &mut f);
            for (_, e) in arms {
                acc = fold_expr(e, acc, &mut f);
            }
        }
        Expr::Rewrite { target, .. } => {
            acc = fold_expr(target, acc, &mut f);
        }
        _ => {}
    }
    acc
}

/// Apply rewrite rules repeatedly until expression stabilises or `max_iters` reached.
pub fn apply_rewrite_fixpoint(mut expr: Expr, rules: &[crate::ast::RewriteRule], max_iters: usize) -> Expr {
    for _ in 0..max_iters {
        let new_expr = apply_rewrite_rules(expr.clone(), rules);
        if new_expr == expr {
            break;
        }
        expr = new_expr;
    }
    expr
}