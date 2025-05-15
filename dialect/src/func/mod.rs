use lorax::{Block, OpBuilder, Operation, Value};

pub fn func(block: Block) -> Operation {
    OpBuilder::new("func.fn").add_block(block).build()
}

pub fn ret<'op>(val: Value) -> Operation {
    OpBuilder::new("func.ret").add_operand(val).build()
}
