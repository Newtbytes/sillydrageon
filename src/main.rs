use clap::{Parser, ValueEnum};

mod driver;

#[derive(Parser)]
struct Cli {
    input: String,

    phase: Option<TestPhase>
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum TestPhase {
    Lex,
    Parse,
    Codegen
}

fn main() {
    let cli: Cli = Cli::parse();
    let input_fn = cli.input;

    // stub
    if let Some(phase) = cli.phase {
        match phase {
            TestPhase::Lex => (),
            TestPhase::Parse => (),
            TestPhase::Codegen => ()
        }

        std::process::exit(0);
    }

    let src = driver::preprocess(&input_fn)
        .expect("Error runnning preprocessor on file.");

    let asm_fn = driver::compile(src);

    driver::assemble(asm_fn)
        .expect("Error assembling compiled program.");

    let _ = driver::cleanup(input_fn);
}
