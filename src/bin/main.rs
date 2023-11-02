use anyhow::Result;
use clap::Parser;

use git_helpe_rs::{
    branch::checkout_to_branch_with_prefix,
    cli_arguments::{CLIArguments, ParsedCLIArguments, ParsedCLIOperationWithArgs},
    commit::commit_with_formatted_message,
    git_config::{BranchOrCommitAction, GitConfig},
};

fn main() -> Result<()> {
    let args: ParsedCLIArguments = CLIArguments::parse().try_into()?;
    println!("args \n {:?}", args);

    let mut config = GitConfig::from_file(args.config_path);

    let resp = match args.operation_with_arguments {
        ParsedCLIOperationWithArgs::Branch(val) => checkout_to_branch_with_prefix(val, config),
        ParsedCLIOperationWithArgs::Commit(val) => commit_with_formatted_message(val, config),
        ParsedCLIOperationWithArgs::SetBranchPrefix(args) => {
            config.set_branch_prefix_variants(args.key, args.value)
        }
        ParsedCLIOperationWithArgs::Show(_) => {
            let config_to_display = config.display_config()?;
            println!("{}", config_to_display);
            Ok(())
        }
        ParsedCLIOperationWithArgs::Delete(_) => todo!("implement me"),
        ParsedCLIOperationWithArgs::SetBranchFormat(args) => {
            config.set_format(BranchOrCommitAction::Branch(args))
        }
        ParsedCLIOperationWithArgs::SetCommitFormat(args) => {
            config.set_format(BranchOrCommitAction::Commit(args))
        }
    };

    match resp {
        Ok(()) => println!("Everything was fine"),
        Err(er) => println!("{:?}", er),
    }

    println!("Hello, world!");
    Ok(())
}
