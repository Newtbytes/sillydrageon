mod ir;
mod link;
mod pool;
mod rewrite;

pub use ir::{
    Block, Constant, OpBuilder, OpResult, Operation, RewritingCtx, Value, Var, rewrite_block,
};
pub use pool::{Pool, Ptr};
pub use rewrite::{RewriteRule, RewriteRuleSet};
