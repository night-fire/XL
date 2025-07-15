pub mod kind;
pub mod ty;
pub mod subst;
pub mod env;
pub mod infer;

pub use kind::Kind;
pub use ty::{Ty, Tv};
pub use infer::{infer_expr, TypeError};