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
}

#[derive(Debug, Clone)]
pub struct BranchOperationArguments {
    branch_prefix_key: String,
}

#[derive(Debug, Clone)]
pub struct CommitOperationArguments {
    interpolation_values: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ParsedCLIOperationWithArgs {
    Branch(BranchOperationArguments),
    Commit(CommitOperationArguments),
}

#[derive(Debug)]
pub struct ParsedCLIArguments {
    operation_with_arguments: ParsedCLIOperationWithArgs,
    config_path: PathBuf,
    use_template: bool,
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
        } else {
            return Err(anyhow!(
                "
            Only valid first arguments are - c (commit) or b (branch) - got {}
            ",
                first_arg
            ));
        };
        match operation {
            Operation::Branch => {
                let prefix_key = value.get(2).expect("Too few arguments");
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
        }
    }
}

// impl TryFrom<Vec<String>

// Interface that I want
// githelpers b f <- making branches happens from clipboard
// githelpers c 2137 "the message that I want to add to commit"

// ==================
// ====  BRANCH  ====
// ==================

// copy it from clipboard -> branch
// apply given prefix from hashmap to branch -> command
// run command -> end

// ==================
// ==== TEMPLATE ====
// ==================

// take interpolation values from CLI
// count interpolation spots number against values provided
//          return error if don't match
// use pattern from config file and interpolate it
// run command
