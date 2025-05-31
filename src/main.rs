use clap::Parser as _;

use sillydrageon::{driver, error::CompilerError};

fn main() -> Result<(), CompilerError> {
    match driver::run_compiler(driver::Cli::parse()) {
        Err(e) => Err(e),
        _ => Ok(()),
    }
}
