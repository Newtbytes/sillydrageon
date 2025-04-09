use clap::Parser;
use driver::ProcFileKind;

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

fn main() {
    let cli: Cli = Cli::parse();
    let input_fn = cli.input;


    let file = driver::ProcFile::from_fn(&input_fn)
        .expect("Error opening source file");

    let src_file = driver::preprocess(file).expect("Error runnning preprocessor on file.");
    let asm_file = src_file.to_kind(ProcFileKind::Assembly);
    let src = src_file.read();

    // tokenization
    let tokens = parser::tokenize(&src);

    if cli.lex {
        dbg!(&tokens);
        std::process::exit(0);
    }

    // parsing
    let ast = parser::parse(&mut parser::tokens(&src));
    let ast = match ast {
        Ok(ref ast) => ast,
        Err(msg) => panic!("{}", msg),
    };

    if cli.parse {
        dbg!(ast);
        std::process::exit(0);
    }

    // codegen
    let asm = codegen::lower(ast);
    let asm = codegen::emit(&asm);
    if cli.codegen {
        println!("{}", asm);
        std::process::exit(0);
    }

    asm_file.write(asm)
        .expect("Error writing assembly to file");

    driver::assemble(asm_file)
        .expect("Error during assembly");
}
