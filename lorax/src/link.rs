use std::{fmt::Display, marker::PhantomData};

use crate::{
    Context,
    ctx::{Pool, Ptr},
};

pub trait LinkedNode {
    fn ahead(&self) -> Option<Ptr>;
    fn behind(&self) -> Option<Ptr>;
    fn ahead_mut(&mut self) -> &mut Option<Ptr>;
    fn behind_mut(&mut self) -> &mut Option<Ptr>;
}

pub struct LinkedListIter<'a, T> {
    ctx: &'a Pool<T>,
    current: Option<Ptr>,
    ty: PhantomData<T>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T>
where
    T: LinkedNode + 'a,
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let curr_ptr = self.current?;
        let node: &T = self.ctx.deref(curr_ptr);
        self.current = node.ahead();
        Some(node)
    }
}

pub trait LinkedList<T: LinkedNode> {
    fn head(&self) -> &Option<Ptr>;
    fn tail(&self) -> &Option<Ptr>;
    fn head_mut(&mut self) -> &mut Option<Ptr>;
    fn tail_mut(&mut self) -> &mut Option<Ptr>;

    fn push(&mut self, ctx: &mut Pool<T>, node: Ptr) -> Ptr {
        if let Some(tail_ptr) = *self.tail_mut() {
            let tail: &mut T = ctx.deref_mut(tail_ptr);
            *tail.ahead_mut() = Some(node);

            let node: &mut T = ctx.deref_mut(node);
            *node.behind_mut() = Some(tail_ptr)
        }

        *self.tail_mut() = Some(node);

        if self.head().is_none() {
            *self.head_mut() = Some(node);
        }

        node
    }

    fn insert_behind(&self, ctx: &mut Pool<T>, root: Ptr, inserted: Ptr) -> Ptr {
        if let Some(behind) = *ctx.deref_mut(root).behind_mut() {
            // link up inserted node between the old behind node and the root
            *ctx.deref_mut(inserted).behind_mut() = Some(behind);
            *ctx.deref_mut(inserted).ahead_mut() = Some(root);

            // the old behind node now points to inserted
            *ctx.deref_mut(behind).ahead_mut() = Some(inserted);

            // point the root's behind ptr to the inserted node
            *ctx.deref_mut(root).behind_mut() = Some(inserted);
        }

        inserted
    }

    fn replace(&self, ctx: &mut Pool<T>, root_ptr: Ptr, mut new: T) {
        let root = ctx.get(root_ptr);

        if let Some(root) = root {
            *new.ahead_mut() = root.ahead();
            *new.behind_mut() = root.behind();

            *ctx.deref_mut(root_ptr) = new;
        }
    }

    fn iter<'a>(&self, ctx: &'a Pool<T>) -> LinkedListIter<'a, T> {
        LinkedListIter {
            ctx: ctx,
            current: *self.head(),
            ty: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Block, Operation, Value, attr::AttributeMap};
    use proptest::prelude::*;

    fn dummy(src: Value, dst: Value) -> Operation {
        Operation {
            name: "test.dummy",
            operands: vec![src],
            blocks: Vec::new(),
            result: dst,
            attributes: AttributeMap::new(),
            behind: None,
            ahead: None,
        }
    }

    fn val() -> Value {
        Value::new(None)
    }

    // #[test]
    // fn push_updates_head_tail() {
    //     let mut bl = Block::new();

    //     assert_eq!(*bl.head(), None);
    //     assert_eq!(*bl.tail(), None);

    //     let ptr = bl.push(dummy(val(), val()));

    //     assert_eq!(*bl.head(), Some(ptr));
    //     assert_eq!(*bl.tail(), Some(ptr));
    // }

    // #[test]
    // fn forward_and_backward_traversal() {
    //     let mut bl = Block::new();
    //     let ptr1 = bl.push(dummy(val(), val()));
    //     let ptr2 = bl.push(dummy(val(), val()));
    //     let ptr3 = bl.push(dummy(val(), val()));

    //     // Forward traversal
    //     let ptrs: Vec<_> = bl.iter().map(|n| n.ahead()).collect();
    //     assert_eq!(ptrs.len(), 3);
    //     // The first node's ahead is Some(ptr2), second is Some(ptr3), third is None
    //     assert_eq!(bl.pool().deref(ptr1).ahead(), Some(ptr2));
    //     assert_eq!(bl.pool().deref(ptr2).ahead(), Some(ptr3));
    //     assert_eq!(bl.pool().deref(ptr3).ahead(), None);

    //     // Backward traversal
    //     assert_eq!(bl.pool().deref(ptr3).behind(), Some(ptr2));
    //     assert_eq!(bl.pool().deref(ptr2).behind(), Some(ptr1));
    //     assert_eq!(bl.pool().deref(ptr1).behind(), None);
    // }

    // #[test]
    // fn insert_behind_head_and_tail() {
    //     let mut bl = Block::new();
    //     let ptr1 = bl.push(dummy(val(), val()));
    //     let ptr2 = bl.push(dummy(val(), val()));
    //     let ptr3 = bl.insert_behind(ptr2, dummy(val(), val()));
    //     // ptr3 should be between ptr1 and ptr2
    //     assert_eq!(bl.pool().deref(ptr1).ahead(), Some(ptr3));
    //     assert_eq!(bl.pool().deref(ptr3).ahead(), Some(ptr2));
    //     assert_eq!(bl.pool().deref(ptr2).behind(), Some(ptr3));
    //     assert_eq!(bl.pool().deref(ptr3).behind(), Some(ptr1));
    // }

    // #[test]
    // fn empty_and_single_element_list() {
    //     let mut bl = Block::new();
    //     assert!(bl.head().is_none());
    //     assert!(bl.tail().is_none());
    //     let ptr = bl.push(dummy(val(), val()));
    //     assert_eq!(bl.head(), bl.tail());
    //     assert_eq!(bl.pool().deref(ptr).ahead(), None);
    //     assert_eq!(bl.pool().deref(ptr).behind(), None);
    // }

    // #[test]
    // fn consistency_of_pointers_after_multiple_ops() {
    //     let mut bl = Block::new();
    //     let ptrs: Vec<_> = (0..10).map(|_| bl.push(dummy(val(), val()))).collect();
    //     // Check forward
    //     for i in 0..9 {
    //         assert_eq!(bl.pool().deref(ptrs[i]).ahead(), Some(ptrs[i + 1]));
    //     }
    //     assert_eq!(bl.pool().deref(ptrs[9]).ahead(), None);
    //     // Check backward
    //     for i in 1..10 {
    //         assert_eq!(bl.pool().deref(ptrs[i]).behind(), Some(ptrs[i - 1]));
    //     }
    //     assert_eq!(bl.pool().deref(ptrs[0]).behind(), None);
    // }

    // proptest! {
    //     #[test]
    //     fn push_many(count in 0usize..10000) {
    //         let mut bl = Block::new();

    //         for _ in 0..count {
    //             let _ = bl.push(dummy(val(), val()));
    //         }

    //         prop_assert_eq!(bl.len(), count);
    //     }
    // }
}
