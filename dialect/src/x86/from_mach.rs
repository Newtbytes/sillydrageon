use lorax::{Context, Ptr};

use super::{
    ops::*,
    state::{rbp, rsp},
};

pub fn lower_frame(ctx: &mut Context, bl_ptr: Ptr, op_ptr: Ptr) {
    let bl = ctx.blocks.deref_mut(bl_ptr);
    let op = ctx.ops.deref(op_ptr);

    if let ("mach.end_frame", &[_]) = (op.name, op.operands.as_slice()) {
        let rbp = bl.insert_behind(&mut ctx.ops, op_ptr, rbp());
        let rsp = bl.insert_behind(&mut ctx.ops, op_ptr, rsp());

        bl.insert_behind(&mut ctx.ops, op_ptr, mov(rbp, rsp));
        bl.replace(&mut ctx.ops, op_ptr, popq(rbp));
    }
}
