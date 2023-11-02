use anyhow::anyhow;
use clap::Parser;
use std::path::PathBuf;

use crate::file_utils::config_file::get_path_to_config;
#[derive(Debug, Parser)]
#[clap()]
pub struct CLIArguments {
    pub args: Vec<String>,

    #[clap(short = 'c', long = "config_path")]
    pub config_path: Option<PathBuf>,

    #[clap(short = 't', long = "use_template", action)]
    pub use_template: bool,
}

pub enum Operation {
    Commit,
    Branch,
    SetCommitFormat,
    SetBranchFormat,
    SetBranchPrefix,
    Delete,
    Show,
}

#[derive(Debug, Clone)]
pub struct BranchOperationArguments {
    pub branch_prefix_key: String,
}

#[derive(Debug, Clone)]
pub struct CommitOperationArguments {
    interpolation_values: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SetOperationArguments {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct DeleteOperationArguments {
    key: String,
}

#[derive(Debug, Clone)]
pub struct ShowOperationArguments {}

#[derive(Debug, Clone)]
pub enum ParsedCLIOperationWithArgs {
    Branch(BranchOperationArguments),
    Commit(CommitOperationArguments),
    SetBranchFormat(SetOperationArguments),
    SetCommitFormat(SetOperationArguments),
    SetBranchPrefix(SetOperationArguments),
    Delete(DeleteOperationArguments),
    Show(ShowOperationArguments),
}

#[derive(Debug)]
pub struct ParsedCLIArguments {
    pub operation_with_arguments: ParsedCLIOperationWithArgs,
    pub config_path: PathBuf,
    pub use_template: bool,
}

impl TryFrom<CLIArguments> for ParsedCLIArguments {
    type Error = anyhow::Error;
    fn try_from(value: CLIArguments) -> Result<Self, Self::Error> {
        let operation: ParsedCLIOperationWithArgs = value.args.try_into()?;
        let config_path = get_path_to_config(value.config_path);

        Ok(ParsedCLIArguments {
            operation_with_arguments: operation,
            config_path: config_path,
            use_template: value.use_template,
        })
    }
}

impl TryFrom<Vec<String>> for ParsedCLIOperationWithArgs {
    type Error = anyhow::Error;
    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.len() < 1 {
            return Err(anyhow!(
                "I need at least one argument - c (commit) or b (branch)"
            ));
        }
        let first_arg = value.get(0).expect("We should never be here");
        let operation = if first_arg == "b" {
            Operation::Branch
        } else if first_arg == "c" {
            Operation::Commit
        } else if first_arg == "set-commit" {
            Operation::SetCommitFormat
        } else if first_arg == "set-branch" {
            Operation::SetBranchFormat
        } else if first_arg == "set-prefix" {
            Operation::SetBranchPrefix
        } else if first_arg == "delete" {
            Operation::Delete
        } else if first_arg == "show" {
            Operation::Show
        } else {
            return Err(anyhow!(
                "
            Only valid first arguments for performing git commands are: \n
            - c (for commit)
            - b (for branch)\n
            Only valid first arguments for other actions are:
            - set-commit
            - set-branch
            - set-prefix
            - delete
            - show
            \n
            Argument provided: {}
            ",
                first_arg
            ));
        };

        let mut value = value;
        match operation {
            Operation::Branch => {
                let prefix_key = value.get(1).expect("Too few arguments");
                Ok(ParsedCLIOperationWithArgs::Branch(
                    BranchOperationArguments {
                        branch_prefix_key: prefix_key.to_string(),
                    },
                ))
            }
            Operation::Commit => {
                let rest_of_args = value[1..].to_vec();
                Ok(ParsedCLIOperationWithArgs::Commit(
                    CommitOperationArguments {
                        interpolation_values: rest_of_args,
                    },
                ))
            }
            // ==============
            // CRUD for config
            Operation::SetBranchFormat => Ok(ParsedCLIOperationWithArgs::SetBranchFormat(
                validate_set_action(&mut value),
            )),
            Operation::SetBranchPrefix => Ok(ParsedCLIOperationWithArgs::SetBranchPrefix(
                validate_set_action(&mut value),
            )),
            Operation::SetCommitFormat => Ok(ParsedCLIOperationWithArgs::SetCommitFormat(
                validate_set_action(&mut value),
            )),
            Operation::Delete => {
                let mut args = value.drain(1..2);
                let key = args
                    .next()
                    .expect("1 argument expected for this operation - provided 0");
                Ok(ParsedCLIOperationWithArgs::Delete(
                    DeleteOperationArguments { key: key },
                ))
            }
            Operation::Show => Ok(ParsedCLIOperationWithArgs::Show(ShowOperationArguments {})),
        }
    }
}

fn validate_set_action(value: &mut Vec<String>) -> SetOperationArguments {
    let mut args = value.drain(1..3);
    let key = args
        .next()
        .expect("At leas two arguments are expected for this operation - provided 0");
    let value = args
        .next()
        .expect("At leas two arguments are expected for this operation - provided 1");
    SetOperationArguments {
        key: key,
        value: value,
    }
}
