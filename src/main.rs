use clap::Parser;

use error::CompilerError;

mod asm;
mod driver;
mod error;
mod parser;
mod src;
mod tictacil;

fn main() -> Result<(), CompilerError> {
    match driver::run_compiler(driver::Cli::parse()) {
        Err(e) => Err(e),
        _ => Ok(()),
    }
}
