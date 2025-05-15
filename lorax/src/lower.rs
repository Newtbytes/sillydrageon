use crate::{Block, Operation, RewriteRule, RewriteRuleSet, Value, Var};

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
