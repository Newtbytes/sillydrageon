use std::sync::atomic;

pub struct Program {
    body: Function,
}

pub struct Function {
    name: String,
    body: Vec<Operation>,
}

#[derive(Clone)]
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
    Const(u32),
    Var(Var),
}

impl From<Var> for Value {
    fn from(var: Var) -> Self {
        Value::Var(var)
    }
}

impl From<u32> for Value {
    fn from(val: u32) -> Self {
        Value::Const(val)
    }
}

pub trait HasResult {
    fn result(&self) -> Var;
}

pub enum Operation {
    Return(Value),
    Unary { op: UnaryOp, src: Value, dst: Var },
}

impl HasResult for Operation {
    fn result(&self) -> Var {
        match self {
            Operation::Unary { op: _, src: _, dst } => dst.clone(),
            _ => unreachable!(),
        }
    }
}

impl HasResult for Vec<Operation> {
    fn result(&self) -> Var {
        // FIXME hacky unwrap()
        self.last().unwrap().result()
    }
}

pub enum UnaryOp {
    Complement,
    Negate,
}
