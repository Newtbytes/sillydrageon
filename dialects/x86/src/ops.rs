use lorax::{Operation, Value, Var};

pub fn mov(src: Value, dst: Var) -> Operation {
    Operation {
        name: "mov".to_owned(),
        operands: vec![src, dst.into()],
        regions: Vec::new(),
        result: Some(Var::new()),
    }
}

pub fn neg(src: Var) -> Operation {
    Operation {
        name: "mov".to_owned(),
        operands: vec![src.into()],
        regions: Vec::new(),
        result: Some(Var::new()),
    }
}
