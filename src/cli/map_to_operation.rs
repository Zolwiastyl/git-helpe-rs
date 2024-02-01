use std::path::PathBuf;

use anyhow::Ok;
use clap::ArgMatches;

use crate::file_utils::config_file::get_path_to_config;

use super::{
    CheckoutToPrefix, CommitOperationArguments, CommitSubcommandFlags, OperationWithArguments,
    ParsedArguments, SetFormat, UseTemplate,
};

impl TryFrom<ArgMatches> for ParsedArguments {
    type Error = anyhow::Error;
    fn try_from(value: ArgMatches) -> Result<Self, Self::Error> {
        let operation_with_arguments = match value.subcommand() {
            Some(("set-branch-prefix", args)) => {
                let format_vals = get_key_val_from_arg_matches(args, "prefix").unwrap();

                Ok(OperationWithArguments::SetBranchPrefix(format_vals))
            }
            Some(("set-branch-template", args)) => {
                let format_vals = get_key_val_from_arg_matches(args, "template").unwrap();

                Ok(OperationWithArguments::SetBranchFormat(format_vals))
            }
            Some(("set-commit", args)) => {
                let format_vals = get_key_val_from_arg_matches(args, "template").unwrap();

                Ok(OperationWithArguments::SetCommitFormat(format_vals))
            }

            Some(("c", args)) => {
                let use_template = get_use_template_from_arg_matches(args);

                let should_use_number_in_branch = args
                    .get_one::<bool>("infer-number-from-branch")
                    .unwrap_or(&false);
                let commit_flags = CommitSubcommandFlags {
                    use_branch_number: should_use_number_in_branch.to_owned(),
                };

                let args = CommitOperationArguments {
                    flags: commit_flags,
                    use_template: use_template,
                };
                let commit_operation_with_arguments = OperationWithArguments::Commit(args);

                Ok(commit_operation_with_arguments)
            }

            Some(("bt", args)) => {
                let use_template = get_use_template_from_arg_matches(args);

                Ok(OperationWithArguments::BranchFromTemplate(use_template))
            }
            Some(("bp", args)) => {
                let prefix_key = args.get_one::<String>("prefix").unwrap();
                let checkout_to_prefix = CheckoutToPrefix {
                    prefix_key: prefix_key.to_owned(),
                };

                Ok(OperationWithArguments::BranchFromClipboard(
                    checkout_to_prefix,
                ))
            }
            Some(("set-clipboard-command", args)) => {
                let mut args = args.clone();
                let clipboard_command: Vec<String> =
                    args.remove_many("copy-paste-pair").unwrap().collect();

                let copy = clipboard_command.get(0).unwrap();
                let paste = clipboard_command.get(1).unwrap();

                Ok(OperationWithArguments::SetClipboardCommands(
                    super::SetClipboardCommands {
                        copy: copy.to_owned(),
                        paste: paste.to_owned(),
                    },
                ))
            }
            Some(("show", _args)) => Ok(OperationWithArguments::Show),
            _ => Err(anyhow::anyhow!("Unknown command")),
        };

        let path_to_config_from_args = value.get_one::<PathBuf>("config");
        let path_to_config = get_path_to_config(path_to_config_from_args.cloned());

        Ok(ParsedArguments {
            operation_with_arguments: operation_with_arguments.unwrap(),
            path_to_config: path_to_config.to_owned(),
        })
    }
}

fn get_key_val_from_arg_matches(
    args: &ArgMatches,
    value_id: &str,
) -> Result<SetFormat, anyhow::Error> {
    let default_key = "default".to_string();
    let key = args.get_one::<String>("key").unwrap_or(&default_key);
    let value = args.get_one::<String>(value_id).unwrap();

    Ok(SetFormat {
        key: key.to_owned(),
        value: value.to_owned(),
    })
}

fn get_use_template_from_arg_matches(args: &ArgMatches) -> UseTemplate {
    let key = if let Some(key) = args.get_one::<String>("key") {
        key.to_owned()
    } else {
        "default".to_owned()
    };

    let mut args = args.clone();

    let interpolate_values: Vec<String> = args.remove_many("interpolate_values").unwrap().collect();

    let use_autocomplete = args.get_one::<bool>("auto-complete").unwrap_or(&false);

    UseTemplate {
        interpolate_values: interpolate_values,
        key: key,
        use_autocomplete: use_autocomplete.to_owned(),
    }
}
