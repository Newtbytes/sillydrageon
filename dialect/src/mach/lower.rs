use lorax::{Context, Ptr, link::LinkedList};

use super::ops::{end_frame, start_frame};

pub fn add_fn_frame(ctx: &mut Context, _: Ptr, op_ptr: Ptr) {
    let op = ctx.ops.deref(op_ptr);

    if op.name == "func.func" {
        if let Some(bl_ptr) = op.blocks.first() {
            let bl = ctx.blocks.deref_mut(*bl_ptr);

            if let (Some(head), Some(tail)) = (*bl.head(), *bl.tail()) {
                let ctx = &mut ctx.ops;

                // FIXME: the frame should start at the entry block and end at the exit block.
                // Once entry/exit blocks exist, this should be handled properly
                let frame = bl.insert_behind(ctx, head, start_frame());
                bl.insert_behind(ctx, tail, end_frame(frame));
            }
        }
    }
}
