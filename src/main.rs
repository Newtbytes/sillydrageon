use std::fs;

use clap::{Parser, ValueEnum};

mod driver;
mod lexer;

#[derive(Parser)]
struct Cli {
    input: String,

    phase: Option<TestPhase>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum TestPhase {
    Lex,
    Parse,
    Codegen,
}

fn main() {
    let cli: Cli = Cli::parse();
    let input_fn = cli.input;

    // stub
    if let Some(phase) = cli.phase {
        match phase {
            TestPhase::Lex => (),
            TestPhase::Parse => (),
            TestPhase::Codegen => (),
        }

        std::process::exit(0);
    }

    let src = driver::preprocess(&input_fn).expect("Error runnning preprocessor on file.");
    let src = fs::read_to_string(src).expect("Error reading preprocessed source code.");

    let tokens = lexer::tokenize(src);

    dbg!(tokens);

    // let asm_fn = driver::compile(src);

    // driver::assemble(asm_fn).expect("Error assembling compiled program.");

    let _ = driver::cleanup(input_fn);
}
