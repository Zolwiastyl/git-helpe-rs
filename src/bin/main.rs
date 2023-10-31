use std::collections::HashMap;

// use crate::cli_arguments::{CLIArguments, ParsedCLIArguments};
use anyhow::Result;
use clap::Parser;

// mod lib;
use git_helpe_rs::{
    branch::checkout_to_branch_with_prefix,
    cli_arguments::{CLIArguments, ParsedCLIArguments},
    git_config::GitConfig,
};
// use crate::branch::checkout_to_branch_with_prefix;
fn main() -> Result<()> {
    let args: ParsedCLIArguments = CLIArguments::parse().try_into()?;
    println!("args, \n {:?}", args);

    let mut some_hashmap = HashMap::new();
    some_hashmap.insert(String::from("f"), String::from("feature/"));
    some_hashmap.insert(String::from("b"), String::from("bugfix/"));
    let config = GitConfig {
        branch_format: String::from(""),
        branch_prefix_variants: some_hashmap,
        commit_format: String::from(""),
    };
    let resp = match args.operation_with_arguments {
        git_helpe_rs::cli_arguments::ParsedCLIOperationWithArgs::Branch(val) => {
            checkout_to_branch_with_prefix(val, config)
        }
        git_helpe_rs::cli_arguments::ParsedCLIOperationWithArgs::Commit(_) => todo!("implement me"),
    };
    match resp {
        Ok(()) => println!("Everything was fine"),
        Err(er) => println!("{:?}", er),
    }

    println!("Hello, world!");
    Ok(())
}
