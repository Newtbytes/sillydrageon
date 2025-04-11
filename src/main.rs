use error::CompilerError;

mod codegen;
mod driver;
mod error;
mod parser;
mod src;

fn main() -> Result<(), CompilerError> {
    match driver::run_compiler() {
        Err(e) => Err(e),
        _ => Ok(()),
    }
}
