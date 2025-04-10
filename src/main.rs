use clap::Parser;
use driver::ProcFileKind;
use error::CompilerError;

mod codegen;
mod driver;
mod error;
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

fn main() -> Result<(), CompilerError> {
    let cli: Cli = Cli::parse();
    let input_fn = cli.input;

    let file = driver::ProcFile::from_fn(&input_fn)
        .ok_or_else(|| CompilerError::ParseError("Invalid source file".to_string()))?;

    let src_file = driver::preprocess(file)?;
    let asm_file = src_file.to_kind(ProcFileKind::Assembly);
    let src = src_file.read()?;

    // tokenization
    let tokens = parser::tokenize(&src)?;

    if cli.lex {
        dbg!(&tokens);
        return Ok(());
    }

    // parsing
    let ast =
        parser::parse(&mut tokens.into_iter()).map_err(|msg| CompilerError::ParseError(msg))?;

    if cli.parse {
        dbg!(ast);
        return Ok(());
    }

    // codegen
    let asm = codegen::lower(&ast);
    let asm = codegen::emit(&asm);
    if cli.codegen {
        println!("{}", asm);
        return Ok(());
    }

    asm_file.write(asm)?;
    driver::assemble(asm_file)?;

    Ok(())
}
