use anyhow::Ok;
use clap::ArgMatches;

pub enum Operation {
    Commit,
    BranchFromClipboard,
    BranchFromTemplate,
    SetCommitFormat,
    SetBranchFormat,
    SetBranchPrefix,
    Show,
}

pub struct SetFormat {
    key: String,
    value: String,
}

pub struct CheckoutToPrefix {
    prefix_key: String,
}

pub struct UseTemplate {
    key: String,
    interpolate_values: Vec<String>,
    use_autocomplete: bool,
}

pub struct CommitSubcommandFlags {
    use_branch_number: bool,
}

pub enum OperationWithArguments {
    Commit(UseTemplate, CommitSubcommandFlags),
    BranchFromClipboard(CheckoutToPrefix),
    BranchFromTemplate(UseTemplate),
    SetCommitFormat(SetFormat),
    SetBranchFormat(SetFormat),
    SetBranchPrefix(SetFormat),
    Show,
}

impl TryFrom<ArgMatches> for OperationWithArguments {
    type Error = anyhow::Error;
    fn try_from(value: ArgMatches) -> Result<Self, Self::Error> {
        return match value.subcommand() {
            Some(("set-branch-prefix", args)) => {
                let format_vals = get_key_val_from_arg_matches(args, "prefix").unwrap();

                Ok(OperationWithArguments::SetBranchPrefix(format_vals))
            }
            Some(("set-branch-format", args)) => {
                let format_vals = get_key_val_from_arg_matches(args, "template").unwrap();

                Ok(OperationWithArguments::SetBranchFormat(format_vals))
            }
            Some(("set-commit-format", args)) => {
                let format_vals = get_key_val_from_arg_matches(args, "template").unwrap();

                Ok(OperationWithArguments::SetCommitFormat(format_vals))
            }

            Some(("commit", args)) => {
                let use_template = get_use_template_from_arg_matches(args);

                let should_use_number_in_branch = args
                    .get_one::<bool>("infer-number-from-branch")
                    .unwrap_or(&false);
                let commit_flags = CommitSubcommandFlags {
                    use_branch_number: should_use_number_in_branch.to_owned(),
                };

                Ok(OperationWithArguments::Commit(use_template, commit_flags))
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
            Some(("show", _args)) => Ok(OperationWithArguments::Show),
            _ => Err(anyhow::anyhow!("Unknown command")),
        };
    }
}

fn get_key_val_from_arg_matches(
    args: &ArgMatches,
    value_id: &str,
) -> Result<SetFormat, anyhow::Error> {
    let key = args.get_one::<String>("key").unwrap();

    let prefix = args.get_one::<String>(value_id).unwrap();

    Ok(SetFormat {
        key: key.to_owned(),
        value: prefix.to_owned(),
    })
}

fn get_use_template_from_arg_matches(args: &ArgMatches) -> UseTemplate {
    let key = if let Some(key) = args.get_one::<String>("key") {
        key.to_owned()
    } else {
        "default".to_owned()
    };

    let interpolate_values = args
        .get_one::<Vec<String>>("interpolate_value")
        .unwrap()
        .to_owned();

    let use_autocomplete = args.get_one::<bool>("auto-complete").unwrap_or(&false);

    UseTemplate {
        interpolate_values: interpolate_values,
        key: key,
        use_autocomplete: use_autocomplete.to_owned(),
    }
}
