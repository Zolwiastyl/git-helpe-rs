use anyhow::Result;
use clap_complete::{generate_to, shells::Bash};
use std::path::PathBuf;

use crate::cli::define::build_cli_commands;

pub fn generate(path: PathBuf) -> Result<()> {
    let mut cmd = build_cli_commands();
    let path = generate_to(Bash, &mut cmd, "git-helpe-rs", path)?;

    println!("completion file has been generated into: {path:?}");

    Ok(())
}
