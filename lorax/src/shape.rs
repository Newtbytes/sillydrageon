use std::{fmt::Display, marker::PhantomData, sync::atomic};

use crate::{
    RewriteRule, RewriteRuleSet,
    pool::{Pool, Ptr},
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
pub struct Operation<'op> {
    pub name: String,
    // pub operands: Vec<Operand>,
    pub operands: Vec<Value>,
    pub blocks: Vec<Block<'op>>,
    pub result: OpResult,
}

impl<'op> Operation<'op> {
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

impl Display for Operation<'_> {
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

pub struct OpBuilder<'op>(Operation<'op>);
impl<'op> OpBuilder<'op> {
    pub fn new(name: &str) -> Self {
        OpBuilder(Operation {
            name: name.to_owned(),
            operands: Vec::new(),
            blocks: Vec::new(),
            result: None,
        })
    }

    pub fn add_operand(mut self, operand: impl Into<Value>) -> Self {
        self.0.operands.push(operand.into());
        self
    }

    pub fn add_block(mut self, block: Block<'op>) -> Self {
        self.0.blocks.push(block);
        self
    }

    pub fn add_result(mut self, result: impl Into<Var>) -> Self {
        self.0.result = Some(result.into());
        self
    }

    pub fn build(self) -> Operation<'op> {
        self.0
    }
}

#[derive(Debug)]
pub struct Block<'pool>
where
    Self: 'pool,
{
    pub(crate) id: usize,
    pub pool: Pool<Operation<'pool>>,
    pub body: Vec<Ptr>,
    phantom: PhantomData<&'pool ()>,
}

impl<'pool> Block<'pool> {
    pub(crate) fn unique_id() -> usize {
        static BLOCK_ID_COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);
        BLOCK_ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed)
    }

    pub fn new(pool: Pool<Operation<'pool>>) -> Self {
        Self {
            id: Self::unique_id(),
            pool: pool,
            body: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn ops(&self) -> impl Iterator<Item = &Ptr> {
        self.body.iter()
    }

    // /// Iterate through all of the sub-blocks
    // /// contained in the operations in this block.
    // pub fn blocks(&'pool self) -> impl Iterator<Item = &'pool Block<'pool>> {
    //     self.body
    //         .iter()
    //         .flat_map(|op| self.pool.get(op).blocks.iter())
    // }

    // pub fn blocks_mut(&'pool mut self) -> impl Iterator<Item = &'pool mut Block<'pool>> {}

    pub fn push(&mut self, op: Operation<'pool>) -> &Operation<'pool> {
        let ptr = self.pool.alloc(op);
        self.body.push(ptr);
        self.pool.get(&ptr)
    }

    pub fn insert(&mut self, idx: usize, op: Operation<'pool>) {
        let ptr = self.pool.alloc(op);
        self.body.insert(idx, ptr);
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }
}

impl Display for Block<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, ".bb{}:", self.id)?;
        for ptr in &self.body {
            writeln!(f, "    {}", self.pool.get(ptr))?;
        }
        Ok(())
    }
}

pub struct RewritingCtx<'ctx> {
    block: &'ctx mut Block<'ctx>,
    op_idx: usize,
}

impl<'block, 'ctx> RewritingCtx<'block>
where
    'block: 'ctx,
    Self: 'ctx,
{
    pub fn new(block: &'block mut Block<'block>) -> Self {
        Self { block, op_idx: 0 }
    }

    fn advance(&mut self) {
        if self.op_idx < self.block.len() {
            self.op_idx += 1;
        }
    }

    pub fn get(&'ctx self) -> &'ctx Operation<'block> {
        self.block
            .body
            .get(self.op_idx)
            .map(|op| self.block.pool.get(op))
            .expect("op_idx should always point to an existing operation")
    }

    pub fn get_mut(&'ctx mut self) -> &'ctx mut Operation<'block> {
        self.block
            .body
            .get_mut(self.op_idx)
            .map(|op| self.block.pool.get_mut(op))
            .expect("op_idx should always point to an existing operation")
    }

    pub fn operands(&'ctx self) -> &'ctx [Value] {
        self.get().operands.as_slice()
    }

    pub fn name(&'ctx self) -> &'ctx str {
        &self.get().name.as_str()
    }

    pub fn result(&'ctx self) -> &'ctx Option<Var> {
        &self.get().result
    }

    pub fn insert_behind(&'ctx mut self, op: Operation<'block>) {
        self.block.insert(self.op_idx, op);

        // adjust index so we continue to point to the correct op
        self.advance();
    }

    pub fn insert_ahead(&'ctx mut self, op: Operation<'block>) {
        self.block.insert(self.op_idx + 1, op);
    }

    pub fn replace(&'ctx mut self, new: Operation<'block>) {
        *(self.get_mut()) = new;
    }

    pub fn done(&'ctx mut self) -> bool {
        self.op_idx >= self.block.len()
    }

    pub fn release(self) -> &'ctx mut Block<'block> {
        self.block
    }
}

pub fn rewrite_block<'a, 'b>(
    block: &'b mut Block<'a>,
    pass: &RewriteRuleSet<RewritingCtx<'a>>,
) -> &'b mut Block<'a>
where
    'b: 'a,
{
    let mut ctx: RewritingCtx<'a> = RewritingCtx::new(block);

    while !ctx.done() {
        pass.apply(&mut ctx);
        ctx.advance();
    }

    ctx.release()
}
