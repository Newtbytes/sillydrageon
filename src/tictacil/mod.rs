use lorax::{Operation, Value, Var};

pub fn ret<V: Into<Value>>(val: V) -> Operation {
    Operation {
        name: "ret".to_owned(),
        operands: val.into(),
        result: None,
    }
}

pub fn neg<V: Into<Value>>(val: V) -> Operation {
    Operation {
        name: "neg".to_owned(),
        operands: val.into(),
        result: Some(Var::new()),
    }
}
