use std::fmt::{self, Display};

use crate::Context;

pub trait Emit {
    fn fmt(&self, ctx: &Context, f: &mut std::fmt::Formatter<'_>) -> fmt::Result;
}

impl<T: Display> Emit for T {
    fn fmt(&self, _: &Context, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self, f)
    }
}

pub struct Emitter<'a, T: Emit> {
    obj: &'a T,
    ctx: &'a Context,
}

impl<'a, T: Emit> Emitter<'a, T> {
    pub fn new(obj: &'a T, ctx: &'a Context) -> Self {
        Self { obj, ctx }
    }
}

impl<T: Emit> Display for Emitter<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.obj.fmt(self.ctx, f)
    }
}
