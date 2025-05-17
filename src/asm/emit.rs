use lorax::{Rewritable, rewrite, rewrite_rule};

use super::nodes::*;

rewrite_rule! {
    Operand => String {
        Operand::Imm(value) => format!("${value}"),
        Operand::Register => "%eax".to_owned(),
    }
}

rewrite_rule! {
    Instruction => String {
        Instruction::Mov { src, dst } => {
            let (src, dst) = (rewrite(src), rewrite(dst));
            format!("movl   {src},{dst}")
        },
        Instruction::Ret => "ret".to_owned()
    }
}

rewrite_rule! {
    Decl => String {
        Decl::Function { name, body } => {
            let mut ins = String::new();

            body.iter().for_each(|x| {
                ins.push_str(&("    ".to_owned() + &rewrite(x) + "\n"));
            });

            // TODO: find a better way to ensure good looking indentation
            format!("\
    .globl {name}

{name}:
{ins}")
        }
    }
}

rewrite_rule! {
    Program => String {
        Program { body } => {
            let body = rewrite(body);

            format!("\
{body}
.section .note.GNU-stack,\"\",@progbits")
        }
    }
}
