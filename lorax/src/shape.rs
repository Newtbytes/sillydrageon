use std::sync::atomic;

#[derive(Clone, Copy)]
pub struct Var {
    id: usize,
}

impl Var {
    pub fn new() -> Self {
        static TMP_ID_COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

        Self {
            id: TMP_ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed),
        }
    }
}

pub enum Value {
    Concrete(i32),
    Symbolic(Var),
}

pub struct Operation {
    name: String,
    operands: Vec<Value>,
    regions: Vec<Region>,
}

pub struct Block {
    pub body: Vec<Operation>,
    var_id_counter: usize,
    args: Vec<Var>,
}

pub struct Region {
    pub body: Vec<Block>,
}

macro_rules! op_rewrite {
    ($name:ident { $($rule:tt)* }) => {
        rewrite_rule! { $name<Operation> { $($rule)* } }
    };
}
