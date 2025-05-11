use std::ops::DerefMut;

pub trait RewriteRule<T> {
    fn apply(&self, node: &mut T);
}

impl<T> RewriteRule<T> for RewriteRuleSet<T> {
    fn apply(&self, node: &mut T) {
        for rule in &self.rules {
            rule.apply(node);
        }
    }
}

impl<T> RewriteRule<Vec<T>> for RewriteRuleSet<T> {
    fn apply(&self, nodes: &mut Vec<T>) {
        for node in nodes {
            self.apply(node);
        }
    }
}

impl<T> RewriteRule<T> for fn(&mut T) {
    fn apply(&self, node: &mut T) {
        self(node)
    }
}

#[derive(Debug)]
pub struct Cursor<'c, T> {
    pub(crate) nodes: &'c mut Vec<T>,
    pub(crate) idx: usize,
}

impl<'c, T> Cursor<'c, T> {
    pub fn new<B>(nodes: &'c mut B) -> Self
    where
        B: DerefMut<Target = Vec<T>>,
    {
        Cursor { nodes, idx: 0 }
    }

    pub fn get(&self) -> Option<&T> {
        self.nodes.get(self.idx)
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.nodes.get_mut(self.idx)
    }

    pub fn push_behind(&mut self, op: T) {
        self.nodes.insert(self.idx, op);
        // adjust index so we continue to point to the correct node
        self.advance();
    }

    pub fn push_ahead(&mut self, op: T) {
        self.nodes.insert(self.idx + 1, op);
    }

    pub fn pos(&self) -> usize {
        return self.idx;
    }

    pub fn advance(&mut self) {
        if self.pos() < self.len() {
            self.idx += 1;
        }
    }

    pub fn replace(&mut self, op: T) {
        *(self.get_mut().unwrap()) = op;
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

/// A collection of rewrite rules, applied in a specific order.
pub struct RewriteRuleSet<T> {
    rules: Vec<Box<dyn RewriteRule<T>>>,
}

impl<T> RewriteRuleSet<T> {
    pub fn new(rules: Vec<Box<dyn RewriteRule<T>>>) -> Self {
        Self { rules }
    }
}

impl<'c, T> RewriteRuleSet<Cursor<'c, T>> {
    pub fn apply<'b, Block>(&self, block: &'b mut Block)
    where
        Block: DerefMut<Target = Vec<T>>,
        'b: 'c,
        T: 'b,
    {
        let mut cursor = Cursor::<'b, T>::new(block);

        while cursor.pos() < cursor.len() {
            for rule in &self.rules {
                rule.apply(&mut cursor);
            }

            cursor.advance();
        }
    }
}
