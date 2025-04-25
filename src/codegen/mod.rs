pub mod asm;

mod emit_asm;
mod lower_ast;
mod tictacil;

use lorax::rewrite;

use super::parser::ast;

// alias
pub fn lower(prg: &ast::Program) -> asm::Program {
    rewrite(prg)
}

pub fn emit(prg: &asm::Program) -> String {
    emit_asm::emit(prg)
}
