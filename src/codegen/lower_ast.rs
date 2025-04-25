use crate::parser::ast;

use super::asm;
use asm::*;

use lorax::{
    rewrite::{Rewritable, rewrite},
    rewrite_rule,
};

rewrite_rule! {
    ast::Expr => asm::Operand {
        ast::Expr::Constant(value) => Operand::Imm(*value),
        ast::Expr::Unary(unary_op, expr) => todo!("Unary expr lowering"),
    }
}

rewrite_rule! {
    ast::Stmt => Vec<asm::Instruction> {
        ast::Stmt::Return(expr) => vec![
            Instruction::Mov {
                src: rewrite(expr),
                dst: Operand::Register,
            },
            Instruction::Ret,
        ],
    }
}

rewrite_rule! {
    ast::Decl => asm::Decl {
        ast::Decl::Function(name, stmt) => Decl::Function {
            name: name.to_string(),
            body: rewrite(stmt.as_ref()),
        },
    }
}

rewrite_rule! {
    ast::Program => asm::Program {
        ast::Program { body } => asm::Program {
            body: rewrite(body),
        },
    }
}
