use clap_complete::{generate_to, shells::Bash};
use std::env;
use std::io::Error;

include!("src/cli/define.rs");

fn main() -> Result<(), Error> {
    println!("asd");
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = build_cli_commands();
    let path = generate_to(
        Bash,
        &mut cmd,       // We need to specify what generator to use
        "git-helpe-rs", // We need to specify the bin name manually
        outdir,         // We need to specify where to write to
    )?;

    println!("cargo:warning=completion file is generated: {path:?}");

    Ok(())
}
