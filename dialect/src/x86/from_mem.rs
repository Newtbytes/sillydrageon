use lorax::{Context, Ptr};

use super::{
    ops::*,
    state::{rbp, rsp},
};

pub fn lower_mem(ctx: &mut Context, bl_ptr: Ptr, op_ptr: Ptr) {
    let bl = ctx.blocks.deref_mut(bl_ptr);
    let op = ctx.ops.deref(op_ptr);

    match (op.name, op.operands.as_slice()) {
        ("mem.alloca", &[size]) => {
            let rsp = bl.insert_behind(&mut ctx.ops, op_ptr, rsp());
            bl.replace(&mut ctx.ops, op_ptr, subq(size, rsp));
        }

        ("mach.end_frame", &[_]) => {
            let rbp = bl.insert_behind(&mut ctx.ops, op_ptr, rbp());
            let rsp = bl.insert_behind(&mut ctx.ops, op_ptr, rsp());

            bl.insert_behind(&mut ctx.ops, op_ptr, mov(rbp, rsp));
            bl.replace(&mut ctx.ops, op_ptr, popq(rbp));
        }
        _ => (),
    }
}
