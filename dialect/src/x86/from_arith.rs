use lorax::{RewriteRule, RewritingCtx};

use super::ops::*;

pub struct LowerBinop;
impl<'block> RewriteRule<RewritingCtx<'block>> for LowerBinop {
    fn apply(&self, ctx: &mut RewritingCtx<'block>) {
        if let (name, &[src], Some(dst)) = (ctx.name(), ctx.operands(), ctx.result()) {
            let ptr = ctx.insert_behind(mov(src, dst));
            let ptr = ctx.deref(ptr).get_result();

            ctx.replace(match name {
                "arith.negate" => neg(ptr),
                "arith.complement" => not(ptr),
                _ => return,
            });
        }
    }
}
