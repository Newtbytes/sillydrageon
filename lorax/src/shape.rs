use std::{
    cell::{RefCell, RefMut},
    fmt::Display,
    sync::atomic,
};

#[derive(Debug, Clone, Copy)]
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

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.id)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Constant {
    pub val: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Var(Var),
    Const(Constant),
}

impl From<Constant> for Value {
    fn from(val: Constant) -> Self {
        Self::Const(val)
    }
}

impl From<Var> for Value {
    fn from(var: Var) -> Self {
        Self::Var(var)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Var(var) => write!(f, "{}", var),
            Value::Const(imm) => write!(f, "{}", imm.val),
        }
    }
}

pub type OpResult = Option<Var>;

#[derive(Debug)]
pub struct Operation {
    pub name: String,
    // pub operands: Vec<Operand>,
    pub operands: Vec<Value>,
    pub regions: Vec<Region>,
    pub result: OpResult,
}

impl Operation {
    pub fn get_result(&self) -> Var {
        self.result
            .expect("this should be called on an op with at least one result")
    }

    pub fn get_mut_result(&mut self) -> &mut Var {
        self.result
            .as_mut()
            .expect("this should be called on an op with at least one result")
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(var) = self.result {
            write!(f, "{} := {} {:?}", var, self.name, self.operands)
        } else {
            write!(f, "{} {:?}", self.name, self.operands)
        }
    }
}

#[derive(Debug)]
pub struct Block {
    pub operations: Vec<Operation>,
}

impl From<Vec<Operation>> for Block {
    fn from(ops: Vec<Operation>) -> Self {
        Self { operations: ops }
    }
}

impl Block {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    pub fn ops(&self) -> impl Iterator<Item = &Operation> {
        self.operations.iter()
    }

    pub fn push(&mut self, op: Operation) -> &Operation {
        self.operations.push(op);

        self.operations
            .last()
            .expect("push always increases length")
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for operation in &self.operations {
            writeln!(f, "{}", operation)?;
        }
        Ok(())
    }
}

// impl From<Block> for Cursor<Operation> {
//     fn from(block: Block) -> Self {
//         Self::new(block.operations)
//     }
// }

#[derive(Debug)]
pub struct Region {
    pub blocks: Vec<Block>,
}

impl Region {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    pub fn ops(&self) -> impl Iterator<Item = &Operation> {
        self.blocks.iter().map(Block::ops).flatten()
    }

    pub fn push(&mut self, block: Block) {
        self.blocks.push(block);
    }
}
