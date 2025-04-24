use std::sync::atomic;

use crate::parser::ast;

struct Program {
    body: Function,
}

struct Function {
    name: String,
    body: Vec<Operation>,
}

#[derive(Clone, Copy)]
struct Tmp {
    id: usize,
}

impl Tmp {
    fn new() -> Self {
        static TMP_ID_COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

        Self {
            id: TMP_ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed),
        }
    }
}

enum Value {
    Const(u32),
    Tmp(Tmp),
}

impl From<Tmp> for Value {
    fn from(tmp: Tmp) -> Self {
        Value::Tmp(tmp)
    }
}

enum Operation {
    Return(Value),
    Unary { op: UnaryOp, src: Value, dst: Tmp },
}

enum UnaryOp {
    Complement,
    Negate,
}

fn from_expr(expr: ast::Expr, ops: &mut Vec<Operation>) -> Value {
    match expr {
        ast::Expr::Constant(val) => Value::Const(val),

        ast::Expr::Unary(op, expr) => {
            let opcode = match op {
                ast::UnaryOp::Complement => UnaryOp::Complement,
                ast::UnaryOp::Negate => UnaryOp::Negate,
            };

            let tmp = Tmp::new();
            let operand = from_expr(*expr, ops);

            ops.push(Operation::Unary {
                op: opcode,
                src: operand,
                dst: tmp,
            });

            Value::from(tmp)
        }
    }
}

fn from_stmt(stmt: ast::Stmt, ops: &mut Vec<Operation>) -> Operation {
    match stmt {
        ast::Stmt::Return(expr) => {
            let val = from_expr(expr, ops);
            Operation::Return(val)
        }
    }
}
