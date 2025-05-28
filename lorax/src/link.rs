use std::marker::PhantomData;

use crate::ctx::{Pool, Ptr};

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
    type Item = Ptr;
    fn next(&mut self) -> Option<Self::Item> {
        let curr_ptr = self.current?;
        let node: &T = self.ctx.deref(curr_ptr);
        self.current = node.ahead();
        Some(curr_ptr)
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
            ctx,
            current: *self.head(),
            ty: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{
        Block, Context, Operation, Ptr, Value,
        attr::AttributeMap,
        link::{LinkedList, LinkedNode},
    };

    use proptest::prelude::*;

    fn dummy(src: Value) -> Operation {
        Operation::new(
            "test.dummy",
            vec![src],
            Vec::new(),
            AttributeMap::new(),
            None,
            None,
        )
    }

    fn val() -> Operation {
        Operation::new(
            "test.dummy_val",
            Vec::new(),
            Vec::new(),
            AttributeMap::new(),
            None,
            None,
        )
    }

    #[test]
    fn push_updates_head_tail() {
        let mut bl = Block::new();
        let mut ctx = Context::new();

        assert_eq!(*bl.head(), None);
        assert_eq!(*bl.tail(), None);

        let c = ctx.ops.alloc(val());
        let ptr = bl.push(&mut ctx, dummy(c.into())).into();

        assert_eq!(*bl.head(), Some(ptr));
        assert_eq!(*bl.tail(), Some(ptr));
    }

    #[test]
    fn forward_and_backward_traversal() {
        let mut bl = Block::new();
        let mut ctx = Context::new();

        let thing = ctx.ops.alloc(val()).into();

        let ptr1: Ptr = bl.push(&mut ctx, dummy(thing)).into();
        let ptr2: Ptr = bl.push(&mut ctx, dummy(thing)).into();
        let ptr3: Ptr = bl.push(&mut ctx, dummy(thing)).into();

        let ctx = &ctx.ops;

        // Forward traversal
        let ptrs: Vec<_> = bl.iter(ctx).collect();
        assert_eq!(ptrs.len(), 3);
        // The first node's ahead is Some(ptr2), second is Some(ptr3), third is None
        assert_eq!(ctx.deref(ptr1).ahead(), ptr2.into());
        assert_eq!(ctx.deref(ptr2).ahead(), Some(ptr3));
        assert_eq!(ctx.deref(ptr3).ahead(), None);

        // Backward traversal
        assert_eq!(ctx.deref(ptr3).behind(), Some(ptr2));
        assert_eq!(ctx.deref(ptr2).behind(), Some(ptr1));
        assert_eq!(ctx.deref(ptr1).behind(), None);
    }

    #[test]
    fn insert_behind_head_and_tail() {
        let mut bl = Block::new();
        let mut ctx = Context::new();

        let thing = ctx.ops.alloc(val()).into();

        let ptr1 = bl.push(&mut ctx, dummy(thing)).into();
        let ptr2 = bl.push(&mut ctx, dummy(thing)).into();

        let ptr3 = (&mut bl)
            .insert_behind(&mut ctx.ops, ptr2, dummy(thing))
            .into();

        // ptr3 should be between ptr1 and ptr2
        assert_eq!(ctx.ops.deref(ptr1).ahead(), Some(ptr3));
        assert_eq!(ctx.ops.deref(ptr3).ahead(), Some(ptr2));
        assert_eq!(ctx.ops.deref(ptr2).behind(), Some(ptr3));
        assert_eq!(ctx.ops.deref(ptr3).behind(), Some(ptr1));
    }

    #[test]
    fn empty_and_single_element_list() {
        let mut bl = Block::new();
        let mut ctx = Context::new();

        assert!(bl.head().is_none());
        assert!(bl.tail().is_none());

        let thing = ctx.ops.alloc(val()).into();

        let ptr = bl.push(&mut ctx, dummy(thing)).into();

        assert_eq!(bl.head(), bl.tail());
        assert_eq!(ctx.ops.deref(ptr).ahead(), None);
        assert_eq!(ctx.ops.deref(ptr).behind(), None);
    }

    #[test]
    fn consistency_of_pointers_after_multiple_ops() {
        let mut bl = Block::new();
        let mut ctx = Context::new();

        let thing = ctx.ops.alloc(val()).into();

        let ptrs: Vec<Ptr> = (0..10)
            .map(|_| bl.push(&mut ctx, dummy(thing)).into())
            .collect();

        let ctx = &ctx.ops;

        // Check forward
        for i in 0..9 {
            assert_eq!(ctx.deref(ptrs[i]).ahead(), Some(ptrs[i + 1]));
        }
        assert_eq!(ctx.deref(ptrs[9]).ahead(), None);
        // Check backward
        for i in 1..10 {
            assert_eq!(ctx.deref(ptrs[i]).behind(), Some(ptrs[i - 1]));
        }
        assert_eq!(ctx.deref(ptrs[0]).behind(), None);
    }

    proptest! {
        #[test]
        fn push_many(count in 0usize..10000) {
            let mut bl = Block::new();
            let mut ctx = Context::new();

            let thing = ctx.ops.alloc(val()).into();

            for _ in 0..count {
                let _ = bl.push(&mut ctx, dummy(thing));
            }

            prop_assert_eq!(bl.iter(&ctx.ops).count(), count);
        }
    }
}
