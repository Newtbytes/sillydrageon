use std::sync::atomic;

pub struct Program {
    body: Function,
}

pub struct Function {
    name: String,
    body: Vec<Operation>,
}

#[derive(Clone, Copy)]
pub struct Tmp {
    id: usize,
}

impl Tmp {
    pub fn new() -> Self {
        static TMP_ID_COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

        Self {
            id: TMP_ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed),
        }
    }
}

pub enum Value {
    Const(u32),
    Tmp(Tmp),
}

impl From<Tmp> for Value {
    fn from(tmp: Tmp) -> Self {
        Value::Tmp(tmp)
    }
}

impl From<u32> for Value {
    fn from(val: u32) -> Self {
        Value::Const(val)
    }
}

pub enum Operation {
    Return(Value),
    Unary { op: UnaryOp, src: Value, dst: Tmp },
}

pub enum UnaryOp {
    Complement,
    Negate,
}
