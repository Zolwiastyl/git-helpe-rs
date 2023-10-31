use crate::cli_arguments::{CLIArguments, ParsedCLIArguments};
use anyhow::Result;
use clap::Parser;

mod cli_arguments;
mod file_utils;
mod git_config;
fn main() -> Result<()> {
    let args: ParsedCLIArguments = CLIArguments::parse().try_into()?;
    println!("args, \n {:?}", args);
    println!("Hello, world!");
    Ok(())
}
