use std::fs;

use clap::Parser;

mod codegen;
mod driver;
mod parser;

#[derive(clap::Parser)]
struct Cli {
    input: String,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    lex: bool,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    parse: bool,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    codegen: bool,
}

fn close(input_fn: &str) {
    let _ = driver::cleanup(input_fn);
    std::process::exit(0);
}

fn main() {
    let cli: Cli = Cli::parse();
    let input_fn = cli.input;

    let src = driver::preprocess(&input_fn).expect("Error runnning preprocessor on file.");
    let src = fs::read_to_string(src).expect("Error reading preprocessed source code.");

    // tokenization
    let tokens = parser::tokenize(&src);

    if cli.lex {
        dbg!(&tokens);
        close(&input_fn);
    }

    // parsing
    let ast = parser::parse(&mut parser::tokens(&src));
    let ast = match ast {
        Ok(ref ast) => ast,
        Err(msg) => panic!("{}", msg),
    };

    if cli.parse {
        dbg!(ast);
        close(&input_fn);
    }

    // codegen
    let asm = codegen::lower_program(&ast);
    if cli.codegen {
        dbg!(asm);
        close(&input_fn);
    }

    // let asm_fn = driver::compile(src);

    // driver::assemble(asm_fn).expect("Error assembling compiled program.");

    close(&input_fn);
}
