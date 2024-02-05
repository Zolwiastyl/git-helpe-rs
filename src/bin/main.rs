use anyhow::Result;

use git_helpe_rs::{
    autocompletion,
    branch::{checkout_to_branch_with_prefix, checkout_to_branch_with_template},
    cli,
    commit::commit_with_formatted_message,
    git_config::GitConfig,
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
            config.set_branch_template_variant(args)
        }
        cli::OperationWithArguments::SetCommitFormat(args) => {
            config.set_commit_template_variant(args)
        }
        cli::OperationWithArguments::BranchFromTemplate(args) => {
            checkout_to_branch_with_template(args, config)
        }
        cli::OperationWithArguments::SetClipboardCommands(args) => {
            config.set_clipboard_command(args)
        }
        cli::OperationWithArguments::GenerateAutocompletionScript(path) => {
            autocompletion::generate(path)
        }
    };

    match resp {
        Ok(()) => {}
        Err(er) => println!("{:?}", er),
    }

    Ok(())
}
