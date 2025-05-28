use std::sync::atomic;

use crate::Emit;
use crate::attr::{Attribute, AttributeMap};
use crate::ctx::Context;
use crate::ctx::{Pool, Ptr};
use crate::link::{LinkedList, LinkedNode};

#[derive(Debug, Clone, Copy)]
pub struct Value {
    ptr: Ptr,
}

impl Value {
    pub fn get_id(&self, ctx: &Context) -> usize {
        ctx.ops.deref(self.ptr).id
    }
}

impl From<Ptr> for Value {
    fn from(ptr: Ptr) -> Self {
        Self { ptr }
    }
}

impl From<Value> for Ptr {
    fn from(val: Value) -> Self {
        val.ptr
    }
}

impl Emit for Value {
    fn fmt(&self, ctx: &Context, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.get_id(ctx))
    }
}

#[derive(Debug)]
pub struct Operation {
    pub name: &'static str,
    id: usize,

    pub operands: Vec<Value>,
    pub blocks: Vec<Ptr>,

    pub attributes: AttributeMap,

    pub behind: Option<Ptr>,
    pub ahead: Option<Ptr>,
}

impl LinkedNode for Operation {
    fn ahead(&self) -> Option<Ptr> {
        self.ahead
    }

    fn behind(&self) -> Option<Ptr> {
        self.behind
    }

    fn ahead_mut(&mut self) -> &mut Option<Ptr> {
        &mut self.ahead
    }

    fn behind_mut(&mut self) -> &mut Option<Ptr> {
        &mut self.behind
    }
}

impl Operation {
    fn unique_id() -> usize {
        static TMP_ID_COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

        TMP_ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed)
    }

    // TODO: replace with an OpBuilder
    pub fn new(
        name: &'static str,

        operands: Vec<Value>,
        blocks: Vec<Ptr>,

        attributes: AttributeMap,

        behind: Option<Ptr>,
        ahead: Option<Ptr>,
    ) -> Self {
        Self {
            name,
            id: Self::unique_id(),
            operands,
            blocks,
            attributes,
            behind,
            ahead,
        }
    }

    pub fn push_block(&mut self, ctx: &mut Context, block: Block) {
        self.blocks.push(ctx.blocks.alloc(block));
    }

    pub fn walk_blocks(&self) -> impl Iterator<Item = &Ptr> {
        self.blocks.iter()
    }

    pub fn walk_blocks_mut(&mut self) -> impl Iterator<Item = &mut Ptr> {
        self.blocks.iter_mut()
    }

    pub fn add_attr(&mut self, key: String, attr: Attribute) {
        self.attributes.insert(key, attr);
    }
}

#[macro_export]
macro_rules! def_op {
    // Block-only operation (no operands, no result)
    ($dl:ident . $name:ident ($field:ident : Block)) => {
        use ::lorax::{Ptr, Operation, attr::AttributeMap};

        pub fn $name($field: Ptr) -> Operation {
            Operation::new(
                stringify!($dl . $name),
                Vec::new(),
                vec![$field],

                AttributeMap::new(),

                None,
                None,
            )
        }
    };

    // Operation with operands, optional result
    ($dl:ident . $name:ident ( $($field:ident : $ty:ty),* $(,)? ) $(-> $ret:ident)? ) => {
        pub fn $name($($field: $ty),*) -> Operation {
            use ::lorax::attr::AttributeMap;

            Operation::new(
                stringify!($dl . $name),
                vec![$($field.into()),*],
                Vec::new(),

                AttributeMap::new(),

                None,
                None,
            )
        }
    };

    // Operation with one attribute
    ($dl:ident . $name:ident (  ) { value: $ty:ty }) => {
        pub fn $name(value: $ty) -> Operation {
            use ::lorax::attr::{AttributeMap, Attribute};

            let mut attributes = AttributeMap::new();
            attributes.insert("value".to_owned(), Attribute::Int(value));

            Operation::new(
                stringify!($dl . $name),
                Vec::new(),
                Vec::new(),

                attributes,

                None,
                None,
            )
        }
    };

    // Attribute map
    (@attr) => {};

    // Result handling
    (@ret) => { Value::new(None) };
    (@ret None) => { Value::new(None) };
    (@ret Value) => { Some(Value::new()) };
    (@ret $ret:ident) => { $ret.into() };
}

fn fmt_delimited_list<I>(
    ctx: &Context,
    list: &mut I,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result
where
    I: Iterator,
    I::Item: Emit,
{
    if let Some(item) = list.next() {
        Emit::fmt(&item, ctx, f)?;
    }

    for item in list {
        write!(f, ", ")?;
        Emit::fmt(&item, ctx, f)?;
    }

    Ok(())
}

impl Emit for Operation {
    fn fmt(&self, ctx: &Context, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{} := {} ", self.id, self.name)?;

        fmt_delimited_list(ctx, &mut self.operands.iter().copied(), f)?;

        if !self.attributes.is_empty() {
            write!(f, "{:?}", self.attributes)?;
        }

        if !self.blocks.is_empty() {
            writeln!(f)?;
        }

        for block in &self.blocks {
            if let Some(block) = ctx.blocks.get(*block) {
                block.fmt(ctx, f)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Block {
    pub(crate) id: usize,

    head: Option<Ptr>,
    tail: Option<Ptr>,
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

impl Block {
    pub(crate) fn unique_id() -> usize {
        static BLOCK_ID_COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);
        BLOCK_ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed)
    }

    pub fn new() -> Self {
        Self {
            id: Self::unique_id(),

            head: None,
            tail: None,
        }
    }

    pub fn push(&mut self, ctx: &mut Context, op: Operation) -> Value {
        let ptr = ctx.ops.alloc(op);
        LinkedList::push(self, &mut ctx.ops, ptr).into()
    }

    pub fn insert_behind(
        &mut self,
        ctx: &mut Pool<Operation>,
        root: Ptr,
        inserted: Operation,
    ) -> Value {
        let inserted = ctx.alloc(inserted);
        LinkedList::insert_behind(self, ctx, root, inserted).into()
    }

    pub fn replace(&self, ctx: &mut Pool<Operation>, root: Ptr, mut new: Operation) {
        new.id = ctx.deref_mut(root).id;
        LinkedList::replace(self, ctx, root, new);
    }
}

impl LinkedList<Operation> for Block {
    fn head(&self) -> &Option<Ptr> {
        &self.head
    }

    fn tail(&self) -> &Option<Ptr> {
        &self.tail
    }

    fn head_mut(&mut self) -> &mut Option<Ptr> {
        &mut self.head
    }

    fn tail_mut(&mut self) -> &mut Option<Ptr> {
        &mut self.tail
    }
}

impl Emit for Block {
    fn fmt(&self, ctx: &Context, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, ".bb{}:", self.id)?;

        for op in self.iter(&ctx.ops) {
            write!(f, "\t")?;
            Emit::fmt(ctx.ops.deref(op), ctx, f)?;
            writeln!(f)?;
        }

        Ok(())
    }
}
