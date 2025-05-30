use crate::{
    Context, Operation, Ptr,
    ctx::{Pool, get_pool},
};

pub struct PoolTransform<Ctx> {
    rewrite: Box<dyn Fn(&mut Ctx, Ptr, Ptr)>,
}

impl PoolTransform<Context> {
    pub fn apply(&self, ctx: &mut Context, block: Ptr, op: Ptr) {
        (self.rewrite)(ctx, block, op)
    }
}

impl<F> From<&'static F> for PoolTransform<Context>
where
    F: Fn(&mut Pool<Operation>, Ptr, Ptr),
{
    fn from(f: &'static F) -> Self {
        Self {
            rewrite: Box::new(|ctx, bl, ptr| f(get_pool(ctx), bl, ptr)),
        }
    }
}

pub struct PassManager {
    rules: Vec<PoolTransform<Context>>,
}

impl Default for PassManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PassManager {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(mut self, rule: impl Fn(&mut Context, Ptr, Ptr) + 'static) -> Self {
        self.rules.push(PoolTransform {
            rewrite: Box::new(rule),
        });
        self
    }

    pub fn apply_one(&self, ctx: &mut Context, ops: Ptr, op: Ptr) {
        for rule in &self.rules {
            rule.apply(ctx, ops, op);
        }
    }
}
