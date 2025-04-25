pub trait Rewritable<T> {
    fn rewrite(node: &Self) -> T;
}

pub fn rewrite<F, T>(node: &F) -> T
where
    F: Rewritable<T>,
{
    Rewritable::rewrite(node)
}

#[macro_export]
macro_rules! rewrite_rule {
    ($from:ty => $to:ty { $($pattern:pat => $result:expr),* $(,)? }) => {
        impl Rewritable<$to> for $from {
            fn rewrite(node: &Self) -> $to {
                match node {
                    $($pattern => $result,)*
                    _ => panic!("Rewrite rule not covered for {:?}", node),
                }
            }
        }
    };
}
