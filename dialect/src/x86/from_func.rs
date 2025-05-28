use lorax::{Block, Operation, Pool, Ptr};

use super::{
    ops::*,
    state::{ax, rbp, rsp},
};

pub fn lower_func(ctx: &mut Pool<Operation>, ops: &mut Block, op_ptr: Ptr) {
    let op = ctx.deref(op_ptr);

    if let ("func.ret", &[val]) = (op.name, op.operands.as_slice()) {
        let v0 = ops.insert_behind(ctx, op_ptr, ax());
        let _ = ops.insert_behind(ctx, op_ptr, mov(val, v0));

        ops.replace(ctx, op_ptr, ret());
    }
}
