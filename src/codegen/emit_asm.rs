use super::asm::*;

fn emit_operand(operand: &Operand) -> String {
    match operand {
        Operand::Imm(value) => format!("${value}"),
        Operand::Register => "%eax".to_owned(),
    }
}

fn emit_instruction(ins: &Instruction) -> String {
    match ins {
        Instruction::Mov { src, dst } => {
            let (src, dst) = (emit_operand(src), emit_operand(dst));
            format!("movl   {src},{dst}")
        }
        Instruction::Ret => "ret".to_owned(),
    }
}

fn emit_decl(decl: &Decl) -> String {
    match decl {
        Decl::Function { name, body } => {
            let mut ins = String::new();

            body.iter().for_each(|x| {
                ins.push_str(&("    ".to_owned() + &emit_instruction(x) + "\n"));
            });

            // TODO: find a better way to ensure good looking indentation
            format!(
                "\
    .globl {name}

{name}:
{ins}"
            )
        }
    }
}

pub fn emit_program(prg: &Program) -> String {
    let body = emit_decl(&prg.body);

    format!(
        "\
{body}
.section .note.GNU-stack,\"\",@progbits"
    )
}
