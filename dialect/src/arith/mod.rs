use lorax::{Operation, Value, Var, def_op};

def_op! {
    arith.negate(val: Value)
}

def_op! {
    arith.complement(val: Value)
}
