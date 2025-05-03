use lorax::{Rewritable, rewrite, rewrite_rule};

use super::nodes as il;
use crate::{parser::ast, tictacil::HasResult};

rewrite_rule! {
    ast::Stmt => il::Operation {
        ast::Stmt::Return(expr) => {
            il::Operation::Return(rewrite(expr))
        }
    }

    ast::Expr => il::Value {
        ast::Expr::Constant(val) => il::Value::Const(*val),
    }

    ast::Expr => Vec<il::Operation> {
        ast::Expr::Unary(op, expr) => {
            let mut src = rewrite::<ast::Expr, Vec<il::Operation>>(expr);

            src.push(il::Operation::Unary {
                op: rewrite(op),
                src: il::Value::Var(src.result()),
                dst: il::Var::new(),
            });

            src
        }
    }

    ast::UnaryOp => il::UnaryOp {
        ast::UnaryOp::Complement => il::UnaryOp::Complement,
        ast::UnaryOp::Negate => il::UnaryOp::Negate,
    }
}
