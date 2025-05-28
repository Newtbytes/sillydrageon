use lorax::{RewriteRule, RewritingCtx};

use super::{ops::*, state::ax};

pub struct LowerFunc;
impl<'block> RewriteRule<RewritingCtx<'block>> for LowerFunc {
    fn apply(&self, ctx: &mut RewritingCtx<'block>) {
        if let ("func.ret", &[val]) = (ctx.name(), ctx.operands()) {
            let v0 = ctx.insert_behind(ax());
            let v0 = ctx.deref(v0).get_result();
            let _ = ctx.insert_behind(mov(val, v0));

            ctx.replace(ret());
        }
    }
}
