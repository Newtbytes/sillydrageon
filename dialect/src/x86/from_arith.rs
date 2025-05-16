use lorax::{RewriteRule, RewritingCtx};

use super::ops::*;

pub struct LowerBinop;
impl<'block> RewriteRule<RewritingCtx<'block>> for LowerBinop {
    fn apply(&self, ctx: &mut RewritingCtx<'block>) {
        match (ctx.name(), ctx.operands(), ctx.result()) {
            (name, &[src], &Some(dst)) => {
                ctx.replace(match name {
                    "arith.negate" => neg(dst.into()),
                    "arith.complement" => todo!("complement"),
                    _ => return (),
                });

                ctx.insert_behind(mov(src, dst.into()));
            }
            _ => (),
        }
    }
}
