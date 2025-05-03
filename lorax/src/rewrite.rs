use std::ops::Deref;

pub trait Rewritable<To> {
    fn rewrite(_: &Self) -> To;
}

pub trait RewriteRule<From, To> {
    fn apply(&self, _: &From) -> To;
}

impl<From, To> RewriteRule<From, To> for From
where
    From: Rewritable<To>,
{
    fn apply(&self, node: &From) -> To {
        From::rewrite(node)
    }
}

pub fn rewrite<From, To>(node: &From) -> To
where
    From: Rewritable<To>,
{
    node.apply(node)
}

#[macro_export]
macro_rules! rewrite_rule {
    ($from:ty => $to:ty { $($pattern:pat => $result:expr),* $(,)? }) => {
        impl Rewritable<$to> for $from {
            fn rewrite(node: &Self) -> $to {
                match node {
                    $($pattern => $result,)*
                    _ => unreachable!("Rewrite rule not covered for {:?}", node),
                }
            }
        }
    };

    ($($from:ty => $to:ty { $($pattern:pat => $result:expr),* $(,)? })+) => {
        $(
            rewrite_rule! { $from => $to { $($pattern => $result),* } }
        )+
    };
}
