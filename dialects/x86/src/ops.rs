use lorax::{Operation, Value, Var};

pub fn mov(src: Value, dst: Var) -> Operation {
    Operation {
        name: "x86.mov".to_owned(),
        operands: vec![src, dst.into()],
        regions: Vec::new(),
        result: Some(Var::new()),
    }
}

pub fn neg(src: Var) -> Operation {
    Operation {
        name: "x86.neg".to_owned(),
        operands: vec![src.into()],
        regions: Vec::new(),
        result: Some(Var::new()),
    }
}
