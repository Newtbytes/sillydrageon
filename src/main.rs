mod codegen;
mod driver;
mod error;
mod parser;
mod src;

fn main() {
    match driver::run_compiler() {
        Err(e) => eprintln!("{}", e),
        _ => {}
    }
}
