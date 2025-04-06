use std::fs;
use std::io;
use std::process::Command;

const CC: &str = "gcc";

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
    let dst_fn = src_fn.replace(".i", "");

    Command::new(CC)
        .arg(src_fn)
        .arg("-o")
        .arg(&dst_fn)
        .output()?;

    return Ok(dst_fn);
}

pub fn cleanup(src_fn: String) -> io::Result<()> {
    let pp_fn = src_fn.replace(".c", ".i");
    let asm_fn = src_fn.replace(".c", ".S");

    if fs::exists(&pp_fn)? {
        fs::remove_file(&pp_fn)?;
    }

    if fs::exists(&asm_fn)? {
        fs::remove_file(&asm_fn)?;
    }

    return Ok(());
}
