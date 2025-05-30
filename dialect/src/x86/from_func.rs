use lorax::{Context, Ptr};

use super::{ops::*, state::ax};

pub fn lower_func(ctx: &mut Context, block: Ptr, op_ptr: Ptr) {
    let block = ctx.blocks.deref_mut(block);
    let op = ctx.ops.deref(op_ptr);

    if let ("func.ret", &[val]) = (op.name, op.operands.as_slice()) {
        let v0 = block.insert_behind(&mut ctx.ops, op_ptr, ax());
        let _ = block.insert_behind(&mut ctx.ops, op_ptr, mov(val, v0));

        block.replace(&mut ctx.ops, op_ptr, ret());
    }
}
