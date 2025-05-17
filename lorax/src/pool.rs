#[derive(Debug, Copy, Clone)]
pub struct Ptr {
    idx: usize,
}

#[derive(Debug)]
pub struct Pool<T> {
    objs: Vec<T>,
}

impl<T> Pool<T> {
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

    pub fn get<'a: 'b, 'b>(&'a self, ptr: &Ptr) -> &'b T {
        self.objs.get(ptr.idx).expect("Deref of dangling ptr")
    }

    pub fn get_mut(&mut self, ptr: &Ptr) -> &mut T {
        self.objs
            .get_mut(ptr.idx)
            .expect("Mut deref of dangling ptr")
    }
}
