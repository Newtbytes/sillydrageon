pub mod asm;

mod emit_asm;
mod lower_ast;
mod tictacil;

use super::parser::ast;

// alias
pub fn lower(prg: &ast::Program) -> asm::Program {
    lower_ast::lower_program(prg)
}

pub fn emit(prg: &asm::Program) -> String {
    emit_asm::emit_program(prg)
}
