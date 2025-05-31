// Lower AST to IR

use lorax::{Block, Context, Value};

use super::ast;

use dialect::{
    arith,
    func::{func, ret},
};

fn lower_expr(ctx: &mut Context, block: &mut Block, expr: &ast::Expr) -> Value {
    let op = match expr {
        ast::Expr::Unary(unary_op, expr) => match unary_op {
            ast::UnaryOp::Complement => todo!(),
            ast::UnaryOp::Negate => arith::negate(lower_expr(ctx, block, expr)),
        },

        ast::Expr::Constant(val) => arith::constant(*val),
    };

    block.push(ctx, op)
}

pub fn lower_stmt(ctx: &mut Context, block: &mut Block, stmt: &ast::Stmt) {
    let op = match stmt {
        ast::Stmt::Return(expr) => ret(lower_expr(ctx, block, expr)),
    };

    block.push(ctx, op);
}

pub fn lower_program(ctx: &mut Context, program: &ast::Program) -> Block {
    let mut region = Block::new();

    match &program.body {
        ast::Decl::Function(_, stmt) => {
            let mut block = Block::new();
            lower_stmt(ctx, &mut block, stmt);

            let block = ctx.blocks.alloc(block);
            region.push(ctx, func(block));
        }
    }

    region
}
