mod ir;
mod pool;
mod rewrite;
mod transform;

pub use ir::{Block, Constant, OpResult, Operation, Value, Var, walk_blocks};
pub use pool::{Pool, Ptr};
pub use rewrite::{RewriteRule, RewriteRuleSet};
pub use transform::{RewritingCtx, rewrite_blocks};
