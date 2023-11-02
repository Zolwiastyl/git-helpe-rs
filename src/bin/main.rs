use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;

use git_helpe_rs::{
    branch::checkout_to_branch_with_prefix,
    cli_arguments::{CLIArguments, ParsedCLIArguments, ParsedCLIOperationWithArgs},
    git_config::GitConfig,
};

fn main() -> Result<()> {
    let args: ParsedCLIArguments = CLIArguments::parse().try_into()?;
    println!("args \n {:?}", args);

    // let mut some_hashmap = HashMap::new();
    // some_hashmap.insert(String::from("f"), String::from("feature/"));
    // some_hashmap.insert(String::from("b"), String::from("bugfix/"));

    // let branch_format = String::from("");
    // let commit_format = String::from("");
    let mut config = GitConfig::from_file(args.config_path);
    // let mut config = GitConfig::new_config(some_hashmap, branch_format, commit_format, None);

    let resp = match args.operation_with_arguments {
        ParsedCLIOperationWithArgs::Branch(val) => checkout_to_branch_with_prefix(val, config),
        ParsedCLIOperationWithArgs::Commit(_) => todo!("implement me"),
        ParsedCLIOperationWithArgs::SetBranchPrefix(args) => {
            config.set_branch_prefix_variants(args.key, args.value)
        }
        ParsedCLIOperationWithArgs::Show(_) => {
            let config_to_display = config.display_config()?;
            println!("{}", config_to_display);
            Ok(())
        }
        ParsedCLIOperationWithArgs::Delete(_) => todo!("implement me"),
        ParsedCLIOperationWithArgs::SetBranchFormat(_) => todo!("implement me"),
        ParsedCLIOperationWithArgs::SetCommitFormat(_) => todo!("implement me"),
    };

    match resp {
        Ok(()) => println!("Everything was fine"),
        Err(er) => println!("{:?}", er),
    }

    println!("Hello, world!");
    Ok(())
}
