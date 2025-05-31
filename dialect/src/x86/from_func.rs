use lorax::{Context, Ptr};

use super::{ops::*, state::ax};

pub fn lower_func(ctx: &mut Context, bl_ptr: Ptr, op_ptr: Ptr) {
    let bl = ctx.blocks.deref_mut(bl_ptr);
    let op = ctx.ops.deref(op_ptr);

    if let ("func.ret", &[val]) = (op.name, op.operands.as_slice()) {
        let v0 = bl.insert_behind(&mut ctx.ops, op_ptr, ax());
        let _ = bl.insert_behind(&mut ctx.ops, op_ptr, mov(val, v0));

        bl.replace(&mut ctx.ops, op_ptr, ret());
    }
}
