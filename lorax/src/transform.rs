use crate::{
    Block, Operation, Ptr,
    ctx::Pool,
    link::{LinkedList, LinkedNode},
};

type Rewriter<Node, List> = fn(&mut Pool<Node>, &mut List, Ptr);

pub struct PoolTransform<Node, List>
where
    List: LinkedList<Node>,
    Node: LinkedNode,
{
    rewrite: Rewriter<Node, List>,
}

impl<Node, List> PoolTransform<Node, List>
where
    List: LinkedList<Node>,
    Node: LinkedNode,
{
    pub fn apply(&self, pool: &mut Pool<Node>, ops: &mut List, op: Ptr) {
        (self.rewrite)(pool, ops, op)
    }
}

impl<List, Node> From<Rewriter<Node, List>> for PoolTransform<Node, List>
where
    List: LinkedList<Node>,
    Node: LinkedNode,
{
    fn from(rewrite: Rewriter<Node, List>) -> Self {
        Self { rewrite }
    }
}

pub struct PassManager {
    rules: Vec<PoolTransform<Operation, Block>>,
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

    pub fn add_rule(mut self, rule: Rewriter<Operation, Block>) -> Self {
        self.rules.push(PoolTransform { rewrite: rule });
        self
    }

    pub fn apply_one(&self, pool: &mut Pool<Operation>, ops: &mut Block, op: Ptr) {
        for rule in &self.rules {
            rule.apply(pool, ops, op);
        }
    }
}
