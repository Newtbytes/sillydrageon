use lorax::{Block, Operation, Pool, Ptr};

use super::ops::*;

pub fn lower_unop(ctx: &mut Pool<Operation>, ops: &mut Block, op_ptr: Ptr) {
    let op = ctx.deref(op_ptr);

    match (op.name, op.operands.as_slice(), op.result) {
        (name, &[src], dst) => {
            ops.insert_behind(ctx, op_ptr, mov(src, dst));

            ops.replace(
                ctx,
                op_ptr,
                match name {
                    "arith.negate" => neg(dst),
                    "arith.complement" => not(dst),
                    _ => return (),
                },
            );
        }
        _ => (),
    }
}
