use std::process::{Command, Stdio};

use anyhow::{Error, Result};

use crate::{cli::DryRunAndCopyFlag, git_config::GitConfig};

pub enum RunMode {
    Normal,
    DryRun,
    DryRunAndCopy,
    Copy,
}

pub fn get_run_mode_from_options(flags: DryRunAndCopyFlag) -> RunMode {
    if flags.copy {
        if flags.dry_run {
            RunMode::DryRunAndCopy
        } else {
            RunMode::Copy
        }
    } else {
        if flags.dry_run {
            RunMode::DryRun
        } else {
            RunMode::Normal
        }
    }
}

pub fn run_copy(config: &GitConfig, value_to_copy: String) -> Result<(), Error> {
    let copy_command = config.data.clipboard_commands.copy.to_string();

    let echo = Command::new("echo")
        .arg(value_to_copy)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    Command::new(copy_command)
        .stdin(Stdio::from(echo.stdout.unwrap()))
        .output()
        .unwrap();

    Ok(())
}
