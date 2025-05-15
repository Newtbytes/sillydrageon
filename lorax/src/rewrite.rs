pub trait RewriteRule<T> {
    fn apply(&self, node: &mut T);
}

/// A collection of rewrite rules, applied in a specific order.
pub struct RewriteRuleSet<T> {
    rules: Vec<Box<dyn RewriteRule<T>>>,
}

impl<T> RewriteRuleSet<T> {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule<R: RewriteRule<T> + 'static>(mut self, rule: R) -> Self {
        self.rules.push(Box::new(rule));
        self
    }
}

impl<T> RewriteRule<T> for RewriteRuleSet<T> {
    fn apply(&self, node: &mut T) {
        for rule in &self.rules {
            rule.apply(node);
        }
    }
}
