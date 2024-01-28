use anyhow::Result;

use git_helpe_rs::{
    branch::checkout_to_branch_with_prefix,
    cli,
    commit::commit_with_formatted_message,
    git_config::{BranchOrCommitAction, GitConfig},
};

fn main() -> Result<()> {
    let args: cli::ParsedArguments = cli::define::build_cli_commands().get_matches().try_into()?;

    let mut config = GitConfig::from_file(args.path_to_config);

    let resp = match args.operation_with_arguments {
        cli::OperationWithArguments::BranchFromClipboard(val) => {
            checkout_to_branch_with_prefix(val, config)
        }
        cli::OperationWithArguments::Commit(val) => commit_with_formatted_message(val, config),
        cli::OperationWithArguments::SetBranchPrefix(args) => {
            config.set_branch_prefix_variant(args.key, args.value)
        }
        cli::OperationWithArguments::Show => {
            let config_to_display = config.display_config()?;
            println!("{}", config_to_display);
            Ok(())
        }
        // TODO implement delete
        // cli::OperationWithArguments::Delete(val) => config.delete_branch_prefix_variant(val.key),
        cli::OperationWithArguments::SetBranchFormat(args) => {
            config.set_format(BranchOrCommitAction::Branch(args))
        }
        cli::OperationWithArguments::SetCommitFormat(args) => {
            config.set_format(BranchOrCommitAction::Commit(args))
        }
        cli::OperationWithArguments::BranchFromTemplate(_args) => {
            todo!("Implement")
        }
        cli::OperationWithArguments::SetClipboardCommand(_) => {
            todo!("Implement")
        }
    };

    match resp {
        Ok(()) => {}
        Err(er) => println!("{:?}", er),
    }

    Ok(())
}
