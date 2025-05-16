use lorax::{Operation, Value, def_op};

def_op! {
    x86.mov(src: Value, dst: Value) -> dst
}

def_op! {
    x86.neg(src: Value) -> None
}

def_op! {
    x86.cmpl(src: Value) -> None
}
