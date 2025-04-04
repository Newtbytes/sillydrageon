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


pub fn preprocess(src_fn: &str) -> io::Result<String> {
    let dst_fn = src_fn.replace(".c", ".i");

    Command::new(CC)
        .arg("-E")
        .arg("-P")
        .arg(src_fn)
        .arg("-o")
        .arg(&dst_fn)

        .output()?;

    return Ok(dst_fn);
}

pub fn compile(src_fn: String) -> String {
    // stub
    return src_fn;
}

pub fn assemble(src_fn: String) -> io::Result<String> {
    let dst_fn = src_fn.replace(".i", ".S");

    Command::new(CC)
        .arg(src_fn)
        .arg("-o")
        .arg(&dst_fn)

        .output()?;

    return Ok(dst_fn);
}

pub fn test_phase(src_fn: &str, phase: CompilePhase) -> Result<(), &'static str> {
    match phase {
        CompilePhase::Tokenization => Ok(()),
        CompilePhase::Parsing => Ok(()),
        CompilePhase::Codegen => Ok(())
    }
}
