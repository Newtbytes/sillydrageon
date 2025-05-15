use lorax::{OpBuilder, Operation, Value, Var};

pub fn negate<'op>(val: Value) -> Operation<'op> {
    OpBuilder::new("arith.neg")
        .add_operand(val)
        .add_result(Var::new())
        .build()
}
