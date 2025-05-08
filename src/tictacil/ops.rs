use lorax::{Operation, Value, Var};

pub fn ret<V: Into<Value>>(val: V) -> Operation {
    Operation {
        name: "ret".to_owned(),
        operands: val.into(),
        result: None,
    }
}

pub fn neg(val: Value) -> Operation {
    Operation {
        name: "neg".to_owned(),
        operands: val,
        result: Some(Var::new()),
    }
}
