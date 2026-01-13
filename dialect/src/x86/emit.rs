use std::fmt::{self, Formatter};

use lorax::{Block, Context, Emit, EmitTarget, Operation, attr::Attribute, link::LinkedList};

pub struct EmitX86 {}
impl EmitTarget for EmitX86 {
    type Ctx = Context;
}

fn emit_operand(op: &Operation) -> String {
    match op.name {
        "arith.constant" => {
            if let Some(Attribute::Int(val)) = op.attributes.get("value") {
                return val.to_string();
            }
        }

        "x86.ax" | "x86.r10" => {
            return op.name.to_owned();
        }

        _ => todo!("emit or handle unkown operation: {}", op.name),
    };

    String::new()
}

impl Emit<EmitX86> for Operation {
    fn emit(&self, ctx: &Context, f: &mut Formatter<'_>) -> fmt::Result {
        self.blocks
            .iter()
            .map(|bl| ctx.blocks.deref(*bl))
            .try_for_each(|bl| <Block as Emit<EmitX86>>::emit(bl, ctx, f))?;

        if let Some(opcode) = self.name.strip_prefix("x86.") {
            write!(f, "{}", opcode)?;

            let operands: Vec<&Operation> = self
                .operands
                .iter()
                .filter_map(|op| op.ptr())
                .filter_map(|op| ctx.ops.get(op))
                .collect();

            write!(f, " ")?;

            match operands.as_slice() {
                [operand] => {
                    write!(f, "{}", emit_operand(operand))?;
                }

                [lhs, rhs] => {
                    write!(f, "{}", emit_operand(lhs))?;
                    write!(f, ", ")?;
                    write!(f, "{}", emit_operand(rhs))?;
                }

                _ => (),
            };
        }

        Ok(())
    }
}

impl Emit<EmitX86> for Block {
    fn emit(&self, ctx: &Context, f: &mut Formatter<'_>) -> fmt::Result {
        for ins in self.iter(&ctx.ops) {
            <Operation as Emit<EmitX86>>::emit(ins, ctx, f)?;
            writeln!(f)?;
        }

        Ok(())
    }
}
