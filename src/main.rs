use std::fs;

use clap::Parser;

mod driver;
mod lexer;
mod parser;

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

fn close(input_fn: &str) {
    let _ = driver::cleanup(input_fn.to_owned());
    std::process::exit(0);
}

fn main() {
    let cli: Cli = Cli::parse();
    let input_fn = cli.input;

    let src = driver::preprocess(&input_fn).expect("Error runnning preprocessor on file.");
    let src = fs::read_to_string(src).expect("Error reading preprocessed source code.");

    let tokens = lexer::tokenize(&src);

    if cli.lex {
        dbg!(tokens);

        close(&input_fn);
    }

    let ast = parser::parse(&mut lexer::tokens(&src));

    if cli.parse {
        match ast {
            Ok(ast) => dbg!(ast),
            Err(msg) => panic!("{}", msg)
        };

        close(&input_fn);
    }

    // let asm_fn = driver::compile(src);

    // driver::assemble(asm_fn).expect("Error assembling compiled program.");

    close(&input_fn);
}
