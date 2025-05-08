// Lower the AST to TICTACIL

use lorax::{Block, Constant, Value};

use super::ast;
use crate::tictacil as il;

fn lower_expr(block: &mut Block, expr: &ast::Expr) -> Value {
    let op = match expr {
        ast::Expr::Unary(unary_op, expr) => match unary_op {
            ast::UnaryOp::Complement => todo!(),
            ast::UnaryOp::Negate => il::neg(lower_expr(block, expr)),
        },

        ast::Expr::Constant(val) => return Constant { val: *val }.into(),
    };

    block.push(op).get_result().into()
}

pub fn lower_stmt(block: &mut Block, stmt: &ast::Stmt) {
    let op = match stmt {
        ast::Stmt::Return(expr) => il::ret(lower_expr(block, expr)),
    };

    block.push(op);
}
