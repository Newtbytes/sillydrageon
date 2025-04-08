pub mod asm;
mod lower_ast;

use super::parser;

// alias
pub fn lower(prg: &parser::Program) -> asm::Program {
    lower_ast::lower_program(&prg)
}
