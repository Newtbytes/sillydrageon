fn synchronize<I, P>(stream: &mut I, predicate: P)
where
    I: Iterator,
    P: Fn(&I::Item) -> bool,
{
    while let Some(item) = stream.next() {
        if predicate(&item) {
            // You can choose either to break with the sync item still unconsumed,
            // or break after consuming it. Here, we break after consuming the item.
            break;
        }
    }
}
