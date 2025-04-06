use std::fs;

use clap::Parser;

mod driver;
mod lexer;

#[derive(Parser)]
struct Cli {
    input: String,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    lex: bool,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    parse: bool,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    codegen: bool,
}

fn main() {
    let cli: Cli = Cli::parse();
    let input_fn = cli.input;

    let src = driver::preprocess(&input_fn).expect("Error runnning preprocessor on file.");
    let src = fs::read_to_string(src).expect("Error reading preprocessed source code.");

    let tokens = lexer::tokenize(src);

    if cli.lex {    
        dbg!(tokens);
        std::process::exit(0);
    }

    // let asm_fn = driver::compile(src);

    // driver::assemble(asm_fn).expect("Error assembling compiled program.");

    let _ = driver::cleanup(input_fn);
}
