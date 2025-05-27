use std::{fmt::Display, sync::atomic};

use crate::Emit;
use crate::attr::{Attribute, AttributeMap};
use crate::ctx::Context;
use crate::ctx::{Pool, Ptr};
use crate::link::{LinkedList, LinkedNode};

#[derive(Debug, Clone, Copy)]
pub struct Value {
    id: usize,
    ptr: Option<Ptr>,
}

impl Value {
    fn unique_id() -> usize {
        static TMP_ID_COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

        TMP_ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed)
    }

    pub fn new(ptr: Option<Ptr>) -> Self {
        Self {
            id: Self::unique_id(),
            ptr,
        }
    }

    pub fn ptr(&self) -> Option<Ptr> {
        self.ptr
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.id)
    }
}

pub type OpResult = Option<Value>;

#[derive(Debug)]
pub struct Operation {
    pub name: &'static str,

    pub operands: Vec<Value>,
    pub blocks: Vec<Ptr>,
    pub result: Value,

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
    pub fn push_block(&mut self, ctx: &mut Context, block: Block) {
        self.blocks.push(ctx.blocks.alloc(block));
    }

    pub fn get_result(&self) -> Value {
        self.result
    }

    pub fn get_mut_result(&mut self) -> &mut Value {
        &mut self.result
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
            Operation {
                name: stringify!($dl . $name),
                operands: Vec::new(),
                blocks: vec![$field],
                result: Value::new(None),

                attributes: AttributeMap::new(),

                behind: None,
                ahead: None,
            }
        }
    };

    // Operation with operands, optional result
    ($dl:ident . $name:ident ( $($field:ident : $ty:ty),* $(,)? ) $(-> $ret:ident)? ) => {
        pub fn $name($($field: $ty),*) -> Operation {
            use ::lorax::attr::AttributeMap;

            Operation {
                name: stringify!($dl . $name),
                operands: vec![$($field.into()),*],
                blocks: Vec::new(),
                result: def_op!(@ret $( $ret )?),

                attributes: AttributeMap::new(),

                behind: None,
                ahead: None,
            }
        }
    };

    // Operation with one attribute
    ($dl:ident . $name:ident (  ) { value: $ty:ty }) => {
        pub fn $name(value: $ty) -> Operation {
            use ::lorax::attr::{AttributeMap, Attribute};

            let mut attributes = AttributeMap::new();
            attributes.insert("value".to_owned(), Attribute::Int(value));

            Operation {
                name: stringify!($dl . $name),
                operands: Vec::new(),
                blocks: Vec::new(),
                result: Value::new(None),

                attributes: attributes,

                behind: None,
                ahead: None,
            }
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

fn fmt_delimited_list<I>(list: &mut I, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
where
    I: Iterator,
    I::Item: Display,
{
    if let Some(item) = list.next() {
        write!(f, "{}", item)?;
    }

    for item in list {
        write!(f, ", {}", item)?;
    }

    Ok(())
}

impl Emit for Operation {
    fn fmt(&self, ctx: &Context, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(_) = self.result.ptr {
            write!(f, "{} := {} ", self.result, self.name)?;
        } else {
            write!(f, "{} ", self.name)?;
        }

        fmt_delimited_list(&mut self.operands.iter(), f)?;

        if !self.attributes.is_empty() {
            write!(f, "{:?}", self.attributes)?;
        }

        if !self.blocks.is_empty() {
            write!(f, "\n")?;
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

    pub fn alloc_op(&mut self, ctx: &mut Pool<Operation>, op: Operation) -> Ptr {
        let ptr = ctx.alloc(op);
        let old_ptr = ctx.deref_mut(ptr).get_mut_result();

        if let Value { id, ptr: None } = old_ptr {
            *old_ptr = Value {
                id: *id,
                ptr: Some(ptr),
            }
        }

        ptr
    }

    pub fn push(&mut self, ctx: &mut Context, op: Operation) -> Value {
        let ptr = self.alloc_op(&mut ctx.ops, op);
        LinkedList::push(self, &mut ctx.ops, ptr);
        ctx.ops.get(ptr).unwrap().result
    }

    pub fn insert_behind(
        &mut self,
        ctx: &mut Pool<Operation>,
        root: Ptr,
        inserted: Operation,
    ) -> Value {
        let inserted = self.alloc_op(ctx, inserted);
        LinkedList::insert_behind(self, ctx, root, inserted);
        ctx.get(inserted).unwrap().result
    }

    pub fn replace(&self, ctx: &mut Pool<Operation>, root: Ptr, mut new: Operation) {
        new.result.id = ctx.deref_mut(root).result.id;
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
            Emit::fmt(op, ctx, f)?;
            write!(f, "\n")?;
        }

        Ok(())
    }
}
