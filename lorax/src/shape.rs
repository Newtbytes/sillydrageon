use std::sync::atomic;

#[derive(Clone, Copy)]
pub struct Tmp {
    id: usize,
}

impl Tmp {
    pub fn new() -> Self {
        static TMP_ID_COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

        Self {
            id: TMP_ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed),
        }
    }
}

enum Operand {
    Imm(i32),
    Tmp(Tmp),
    Region(Region),
}

struct Operation {
    dst: Tmp,
    lhs: Operand,
    rhs: Operand,
}

struct Block {
    body: Vec<Operation>,
}

struct Region {
    body: Vec<Block>,
}
