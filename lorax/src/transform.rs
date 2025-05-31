use crate::{Context, Ptr};

pub struct PoolTransform<Ctx> {
    rewrite: Box<dyn Fn(&mut Ctx, Ptr, Ptr)>,
}

impl PoolTransform<Context> {
    pub fn apply(&self, ctx: &mut Context, block: Ptr, op: Ptr) {
        (self.rewrite)(ctx, block, op)
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
