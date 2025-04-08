mod asm;
pub use asm::*;

use super::parser;

fn lower_expr(expr: parser::Expr) -> asm::Operand {
    match expr {
        parser::Expr::Constant(value) => Operand::Imm(value),
    }
}

fn lower_stmt(stmt: parser::Stmt) -> Vec<asm::Instruction> {
    match stmt {
        parser::Stmt::Return(expr) => vec![
            Instruction::Mov {
                src: lower_expr(expr),
                dst: Operand::Register,
            },
            Instruction::Ret,
        ],
    }
}

fn lower_decl(stmt: parser::Decl) -> asm::Decl {
    match stmt {
        parser::Decl::Function(name, stmt) => Decl::Function {
            name,
            body: lower_stmt(*stmt),
        },
    }
}

fn lower_program(prg: parser::Program) -> asm::Program {
    asm::Program {
        body: vec![lower_decl(prg.body)],
    }
}
