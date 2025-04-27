use super::nodes as il;
use crate::parser::ast;

fn from_expr(expr: ast::Expr, ops: &mut Vec<il::Operation>) -> il::Value {
    match expr {
        ast::Expr::Constant(val) => il::Value::Const(val),

        ast::Expr::Unary(op, expr) => {
            let opcode = match op {
                ast::UnaryOp::Complement => il::UnaryOp::Complement,
                ast::UnaryOp::Negate => il::UnaryOp::Negate,
            };

            let tmp = il::Tmp::new();
            let operand = from_expr(*expr, ops);

            ops.push(il::Operation::Unary {
                op: opcode,
                src: operand,
                dst: tmp,
            });

            il::Value::from(tmp)
        }
    }
}

fn from_stmt(stmt: ast::Stmt, ops: &mut Vec<il::Operation>) -> il::Operation {
    match stmt {
        ast::Stmt::Return(expr) => {
            let val = from_expr(expr, ops);
            il::Operation::Return(val)
        }
    }
}
