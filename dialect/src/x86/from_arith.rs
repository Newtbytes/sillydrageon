use lorax::{Context, Ptr};

use super::ops::{mov, neg, not};

pub fn lower_unop(ctx: &mut Context, bl_ptr: Ptr, op_ptr: Ptr) {
    let bl = ctx.blocks.deref_mut(bl_ptr);
    let op = ctx.ops.deref(op_ptr);

    if let (name, &[src], dst) = (op.name, op.operands.as_slice(), op.result) {
        let op = match name {
            "arith.negate" => neg(dst),
            "arith.complement" => not(dst),
            _ => return,
        };

        bl.insert_behind(&mut ctx.ops, op_ptr, mov(src, dst));

        bl.replace(&mut ctx.ops, op_ptr, op);
    }
}
