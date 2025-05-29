use std::fmt::{self, Display, Formatter};

use crate::Context;

pub trait EmitTarget {
    type Ctx;
}

pub struct EmitIR {}
impl EmitTarget for EmitIR {
    type Ctx = Context;
}

pub trait Emit<T: EmitTarget> {
    fn emit(&self, ctx: &<T as EmitTarget>::Ctx, f: &mut Formatter<'_>) -> fmt::Result;
}

pub struct Emitter<'a, Obj, Target: EmitTarget> {
    pub(crate) ctx: &'a Target::Ctx,
    pub(crate) obj: &'a Obj,
}

impl<Obj: Emit<Target>, Target: EmitTarget> Display for Emitter<'_, Obj, Target> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.obj.emit(self.ctx, f)
    }
}

pub fn emit<'a, Obj, Target: EmitTarget>(
    ctx: &'a Target::Ctx,
    obj: &'a Obj,
) -> Emitter<'a, Obj, Target> {
    Emitter { ctx, obj }
}
