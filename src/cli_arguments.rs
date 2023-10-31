use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap()]
pub struct CLIArguments {
    pub args: Vec<String>,

    #[clap(short = 'c', long = "config_path")]
    pub config_path: Option<PathBuf>,
}

// Interface that I want
// githelpers b f
// githelpers c 2137 "the message that I want to add to commit"
