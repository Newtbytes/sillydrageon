use clap::Parser;

use error::CompilerError;

mod driver;
mod error;
mod parser;
mod src;

fn main() -> Result<(), CompilerError> {
    match driver::run_compiler(driver::Cli::parse()) {
        Err(e) => Err(e),
        _ => Ok(()),
    }
}
