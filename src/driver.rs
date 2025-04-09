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

    Ok(dst_fn)
}

pub fn assemble(asm: &str, src_fn: &str) -> io::Result<String> {
    let src_fn = src_fn.replace(".c", ".S");
    let dst_fn = src_fn.replace(".S", "");
    fs::write(&src_fn, asm)?;

    Command::new(CC)
        .arg(&src_fn)
        .arg("-o")
        .arg(&dst_fn)
        .output()?;

    fs::remove_file(&src_fn)?;

    Ok(dst_fn)
}

pub fn cleanup(src_fn: &str) -> io::Result<()> {
    let pp_fn = src_fn.replace(".c", ".i");
    let asm_fn = src_fn.replace(".c", ".S");

    if fs::exists(&pp_fn)? {
        fs::remove_file(&pp_fn)?;
    }

    if fs::exists(&asm_fn)? {
        fs::remove_file(&asm_fn)?;
    }

    Ok(())
}
