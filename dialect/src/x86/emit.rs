use std::fmt::{self, Formatter};

use lorax::{Block, Context, Emit, EmitTarget, Operation};

pub struct EmitX86 {}
impl EmitTarget for EmitX86 {
    type Ctx = Context;
}

impl Emit<EmitX86> for Operation {
    fn emit(&self, ctx: &Context, f: &mut Formatter<'_>) -> fmt::Result {
        todo!("emit x86 assembly for lorax::Operation")
    }
}

impl Emit<EmitX86> for Block {
    fn emit(&self, ctx: &Context, f: &mut Formatter<'_>) -> fmt::Result {
        todo!("emit x86 assembly for lorax::Block")
    }
}
