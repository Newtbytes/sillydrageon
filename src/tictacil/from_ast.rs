use lorax::{Block, Constant, Value};

use crate::parser::ast;

use super::ops;

fn lower_expr(block: &mut Block, expr: &ast::Expr) -> Value {
    let op = match expr {
        ast::Expr::Unary(unary_op, expr) => match unary_op {
            ast::UnaryOp::Complement => todo!(),
            ast::UnaryOp::Negate => ops::neg(lower_expr(block, expr)),
        },

        ast::Expr::Constant(val) => return Constant { val: *val }.into(),
    };

    block.push(op).into()
}

pub fn lower_stmt(block: &mut Block, stmt: &ast::Stmt) {
    let op = match stmt {
        ast::Stmt::Return(expr) => ops::ret(lower_expr(block, expr)),
    };

    block.push(op);
}
