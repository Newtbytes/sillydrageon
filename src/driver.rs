use std::fs;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::process::Command;


const CC: &str = "gcc";


pub enum CompilePhase {
    Tokenization,
    Parsing,
    Codegen
}


fn preprocess(src_fn: &str) -> io::Result<String> {
    let dst_fn = src_fn.replace(".c", ".i");

    Command::new(CC)
        .arg("-E")
        .arg("-P")
        .arg(src_fn)
        .arg("-o")
        .arg(&dst_fn)

        .output()?;

    let mut f = File::open(&dst_fn)?;

    let mut src = String::new();
    f.read_to_string(&mut src)?;

    fs::remove_file(dst_fn)?;

    return Ok(src);
}

fn compile(src: String) -> String {
    // stub
    return src;
}

fn assemble(src_fn: &str, dst_fn: &str) -> io::Result<()> {
    assert!(src_fn.ends_with(".s"));

    Command::new(CC)
        .arg(src_fn)
        .arg("-o")
        .arg(dst_fn)

        .output()?;

    return Ok(());
}

pub fn test_phase(src_fn: &str, phase: CompilePhase) -> Result<(), &'static str> {
    match phase {
        CompilePhase::Tokenization => Ok(()),
        CompilePhase::Parsing => Ok(()),
        CompilePhase::Codegen => Ok(())
    }
}
