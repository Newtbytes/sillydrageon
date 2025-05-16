use lorax::{Block, Operation, Value, def_op};

def_op! {
    func.func(block: Block)
}

def_op! {
    func.ret(val: Value) -> None
}
