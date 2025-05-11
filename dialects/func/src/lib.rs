use lorax::{Block, Operation, Value};

pub fn func(block: Block) -> Operation {
    Operation {
        name: "fn".to_owned(),
        operands: Vec::new(),
        blocks: vec![block],
        result: None,
    }
}

pub fn ret<V: Into<Value>>(val: V) -> Operation {
    Operation {
        name: "ret".to_owned(),
        operands: vec![val.into()],
        blocks: Vec::new(),
        result: None,
    }
}
