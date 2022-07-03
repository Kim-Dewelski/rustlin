pub mod targets;

use anyhow::{anyhow, Result};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Default)]
pub struct LinkFlags {}

#[derive(Default)]
pub struct CompileFlags {
    pub release: bool,
    pub target: Option<targets::Targets>,
}

pub struct Metadata {
    pub executables: Vec<PathBuf>,
}

pub fn build(
    crate_path: &Path,
    link_flags: LinkFlags,
    compile_flags: CompileFlags,
) -> Result<Metadata> {
    let mut cargo = Command::new("cargo");
    cargo.arg("build");
    cargo.current_dir(crate_path);
    if compile_flags.release {
        cargo.arg("--release");
    }
    if let Some(target) = compile_flags.target {
        cargo.arg("--target").arg(target.to_string());
    }
    let output = cargo
        .output()
        .expect(&format!("Unable to execute cargo build command {cargo:?}"));
    if !output.status.success() {
        return Err(anyhow!(
            "Cargo build command: '{cargo:?}' failed to execute\n\r{}",
            String::from_utf8(output.stderr)
                .expect("Unable to translate error message into string")
        ));
    }
    cargo.arg("--message-format").arg("json");
    let output = cargo.output().expect(&format!(
        "Unable to execute cargo build command for json output {cargo:?}"
    ));
    let stdout =
        String::from_utf8(output.stdout).expect("Unable to convert stdout output into string");
    let mut executables = vec![];
    for line in stdout.lines() {
        let mut json = json::parse(line).expect("Unable to parse json output from cargo");
        if let Some(val) = json["executable"].take_string() {
            executables.push(PathBuf::from(val));
        }
    }
    Ok(Metadata { executables })
}
