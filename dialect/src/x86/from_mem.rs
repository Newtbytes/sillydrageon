use lorax::{Block, Operation, Pool, Ptr};

use super::{
    ops::*,
    state::{rbp, rsp},
};

pub fn lower_mem(ctx: &mut Pool<Operation>, bl: &mut Block, op_ptr: Ptr) {
    let op = ctx.deref(op_ptr);

    match (op.name, op.operands.as_slice()) {
        ("mem.alloca", &[size]) => {
            let rsp = bl.insert_behind(ctx, op_ptr, rsp());
            bl.replace(ctx, op_ptr, subq(size, rsp));
        }

        // function epilogue
        ("x86.ret", _) => {
            let rbp = bl.insert_behind(ctx, op_ptr, rbp());
            let rsp = bl.insert_behind(ctx, op_ptr, rsp());

            bl.insert_behind(ctx, op_ptr, mov(rbp, rsp));
            bl.insert_behind(ctx, op_ptr, popq(rbp));
        }
        _ => (),
    }
}
