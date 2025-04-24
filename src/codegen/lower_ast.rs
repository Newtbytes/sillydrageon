use crate::parser::ast;

use super::asm;
use asm::*;

fn lower_expr(expr: &ast::Expr) -> asm::Operand {
    match expr {
        ast::Expr::Constant(value) => Operand::Imm(*value),
        ast::Expr::Unary(unary_op, expr) => todo!("Unary expr lowering"),
    }
}

fn lower_stmt(stmt: &ast::Stmt) -> Vec<asm::Instruction> {
    match stmt {
        ast::Stmt::Return(expr) => vec![
            Instruction::Mov {
                src: lower_expr(expr),
                dst: Operand::Register,
            },
            Instruction::Ret,
        ],
    }
}

fn lower_decl(stmt: &ast::Decl) -> asm::Decl {
    match stmt {
        ast::Decl::Function(name, stmt) => Decl::Function {
            name: name.to_string(),
            body: lower_stmt(stmt),
        },
    }
}

pub fn lower_program(prg: &ast::Program) -> asm::Program {
    asm::Program {
        body: lower_decl(&prg.body),
    }
}
