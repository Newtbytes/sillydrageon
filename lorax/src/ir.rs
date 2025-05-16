use std::{fmt::Display, sync::atomic};

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
    pub name: &'static str,
    pub operands: Vec<Value>,
    pub blocks: Vec<Block>,
    pub result: OpResult,
}

impl Operation {
    pub fn push_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn get_result(&self) -> Var {
        self.result
            .expect("this should be called on an op with at least one result")
    }

    pub fn get_mut_result(&mut self) -> &mut Var {
        self.result
            .as_mut()
            .expect("this should be called on an op with at least one result")
    }

    pub fn walk_blocks(&mut self) -> impl Iterator<Item = &mut Block> {
        self.blocks.iter_mut()
    }
}

#[macro_export]
macro_rules! def_op {
    // Block-only operation (no operands, no result)
    ($dl:ident . $name:ident ($field:ident : Block)) => {
        pub fn $name($field: Block) -> Operation {
            Operation {
                name: stringify!($dl . $name),
                operands: Vec::new(),
                blocks: vec![$field],
                result: None,
            }
        }
    };

    // Operation with operands, optional result
    ($dl:ident . $name:ident ( $($field:ident : $ty:ty),* $(,)? ) $(-> $ret:ident)? ) => {
        pub fn $name($($field: $ty),*) -> Operation {
            Operation {
                name: stringify!($dl . $name),
                operands: vec![$($field.into()),*],
                blocks: Vec::new(),
                result: def_op!(@ret $( $ret )?),
            }
        }
    };

    // Result handling
    (@ret) => { Some(Var::new()) };
    (@ret None) => { None };
    (@ret Var) => { Some(Var::new()) };
    (@ret $ret:ident) => { Some(($ret).into()) };
}

fn fmt_delimited_list<I>(list: &mut I, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
where
    I: Iterator,
    I::Item: Display,
{
    if let Some(item) = list.next() {
        write!(f, "{}", item)?;
    }

    for item in list {
        write!(f, ", {}", item)?;
    }

    Ok(())
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(var) = self.result {
            write!(f, "{} := {} ", var, self.name)?;
        } else {
            write!(f, "{} ", self.name)?;
        }

        fmt_delimited_list(&mut self.operands.iter(), f)?;

        if !self.blocks.is_empty() {
            write!(f, "\n")?;
        }

        for block in &self.blocks {
            write!(f, "{}", block)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Block {
    pub(crate) id: usize,
    pub body: Vec<Operation>,
}

impl Block {
    pub(crate) fn unique_id() -> usize {
        static BLOCK_ID_COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);
        BLOCK_ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed)
    }

    pub fn new() -> Self {
        Self {
            id: Self::unique_id(),
            body: Vec::new(),
        }
    }

    pub fn get(&self, idx: usize) -> &Operation {
        self.body
            .get(idx)
            .expect("idx should always point to an existing operation")
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut Operation {
        self.body
            .get_mut(idx)
            .expect("idx should always point to an existing operation")
    }

    pub fn walk_ops(&mut self) -> impl Iterator<Item = &mut Operation> {
        self.body.iter_mut()
    }

    pub fn push(&mut self, op: Operation) -> &Operation {
        self.body.push(op);
        self.body.last().expect("last op should always exist")
    }

    pub fn insert(&mut self, idx: usize, op: Operation) {
        self.body.insert(idx, op);
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, ".bb{}:", self.id)?;
        for op in &self.body {
            writeln!(f, "    {}", op)?;
        }
        Ok(())
    }
}

// this is incorect, but for now it will do
pub fn walk_blocks<'a>(block: &'a mut Block) -> Box<dyn Iterator<Item = &'a mut Block> + 'a> {
    let mut blocks = Vec::new();

    for op in block.walk_ops() {
        blocks.extend(op.walk_blocks());
    }

    Box::new(blocks.into_iter())
}
