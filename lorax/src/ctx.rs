use std::ops::Index;

use crate::{Block, Operation};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Ptr {
    pub(crate) idx: usize,
}

impl Ptr {
    pub fn new(idx: usize) -> Self {
        Self { idx }
    }
}

impl From<usize> for Ptr {
    fn from(idx: usize) -> Self {
        Self { idx }
    }
}

#[derive(Debug)]
pub struct Pool<T> {
    objs: Vec<T>,
}

impl<'a, T> Default for Pool<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Pool<T> {
    pub fn new() -> Self {
        Pool { objs: Vec::new() }
    }

    pub fn reserve(&mut self, count: usize) {
        self.objs.reserve(count);
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Pool {
            objs: Vec::with_capacity(capacity),
        }
    }

    pub fn alloc(&mut self, obj: T) -> Ptr {
        self.objs.push(obj);

        Ptr {
            idx: self.objs.len() - 1,
        }
    }

    pub fn get(&self, ptr: Ptr) -> Option<&T> {
        self.objs.get(ptr.idx)
    }

    pub fn get_mut(&mut self, ptr: Ptr) -> Option<&mut T> {
        self.objs.get_mut(ptr.idx)
    }

    pub fn deref(&'a self, ptr: Ptr) -> &'a T {
        self.get(ptr).expect("Deref of dangling ptr")
    }

    pub fn deref_mut(&'a mut self, ptr: Ptr) -> &'a mut T {
        self.get_mut(ptr).expect("Mut deref of dangling ptr")
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.objs.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.objs.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.objs.len()
    }
}

impl<T> Index<usize> for Pool<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.objs[index]
    }
}

pub struct Context
where
    Self: ContextImpl<Block> + ContextImpl<Operation>,
{
    pub blocks: Pool<Block>,
    pub ops: Pool<Operation>,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            blocks: Pool::new(),
            ops: Pool::new(),
        }
    }
}

pub trait ContextImpl<T> {
    fn get_pool(&mut self) -> &mut Pool<T>;
}

impl ContextImpl<Block> for Context {
    fn get_pool(&mut self) -> &mut Pool<Block> {
        &mut self.blocks
    }
}

impl ContextImpl<Operation> for Context {
    fn get_pool(&mut self) -> &mut Pool<Operation> {
        &mut self.ops
    }
}

pub fn get_pool<T, U: ContextImpl<T>>(ctx: &mut U) -> &mut Pool<T> {
    ctx.get_pool()
}
