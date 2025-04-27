mod emit;
mod from_ast;
mod nodes;

pub use emit::*;
pub use from_ast::*;
pub use nodes::*;

use crate::parser::ast;
use lorax::rewrite;

pub fn lower_ast(prg: &ast::Program) -> Program {
    rewrite(prg)
}

pub fn emit(prg: &Program) -> String {
    rewrite(prg)
}
