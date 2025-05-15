use lorax::{OpBuilder, Operation, Value, Var};

pub fn mov<'op>(src: Value, dst: Var) -> Operation<'op> {
    OpBuilder::new("x86.mov")
        .add_operand(src)
        .add_result(dst)
        .build()
}

pub fn neg<'op>(src: Var) -> Operation<'op> {
    OpBuilder::new("x86.neg").add_operand(src).build()
}
