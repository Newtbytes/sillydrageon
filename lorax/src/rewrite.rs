struct RuleMatch<T> {
    node: T,
}

pub trait RewriteRule<T> {
    fn matches(&self, node: &T) -> Option<RuleMatch<T>>;
    fn rewrite(&self, node: &RuleMatch<T>) -> T;

    fn apply(&self, node: &T) -> Option<T> {
        if let Some(matched) = self.matches(&node) {
            Some(self.rewrite(&matched))
        } else {
            None
        }
    }
}

#[macro_export]
macro_rules! rewrite_rule {
    (@rules $($lhs:pat => $rhs:expr),* $(,)?) => {
        match node {
            $($lhs => Some(RuleMatch { node: $rhs }),)*
            _ => None,
        }
    }

    ($name:ident<$node:ty> { $($rule:tt)* }) => {
        strict $name;
        impl RewriteRule<$node> for $name {
            fn matches(&self, node: &$node) -> Option<RuleMatch<$node>> {
                match node {
                    $($lhs => Some(RuleMatch { node: $rhs }),)*
                    _ => None,
                }
            }

            fn rewrite(&self, node: &RuleMatch<$node>) -> $node {
                match node.node {
                    $($lhs => $rhs,)*
                    _ => unreachable!(),
                }
            }
        }
    }
}
