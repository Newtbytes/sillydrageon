use clap::Parser;

use error::CompilerError;
use parser::ast;

//mod asm;
//mod driver;
mod error;
mod parser;
mod src;
mod tictacil;

fn main() -> () {
    let mut block = lorax::Block {
        operations: Vec::new(),
    };

    let stmt = ast::Stmt::Return(ast::Expr::Unary(
        ast::UnaryOp::Negate,
        Box::new(ast::Expr::Constant(1)),
    ));

    let _ = parser::lower_stmt(&mut block, &stmt);
    println!("{}", block);
    // match driver::run_compiler(driver::Cli::parse()) {
    //     Err(e) => Err(e),
    //     _ => Ok(()),
    // }
}
