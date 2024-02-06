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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_run_mode_from_options() {
        // Test case: Copy flag is true, DryRun flag is true
        let flags = DryRunAndCopyFlag {
            copy: true,
            dry_run: true,
        };
        let run_mode = get_run_mode_from_options(flags);
        match run_mode {
            RunMode::DryRunAndCopy => assert!(true),
            _ => assert!(false),
        }

        // Test case: Copy flag is true, DryRun flag is false
        let flags = DryRunAndCopyFlag {
            copy: true,
            dry_run: false,
        };
        let run_mode = get_run_mode_from_options(flags);
        match run_mode {
            RunMode::Copy => assert!(true),
            _ => assert!(false),
        }

        // Test case: Copy flag is false, DryRun flag is true
        let flags = DryRunAndCopyFlag {
            copy: false,
            dry_run: true,
        };
        let run_mode = get_run_mode_from_options(flags);
        match run_mode {
            RunMode::DryRun => assert!(true),
            _ => assert!(false),
        }

        // Test case: Copy flag is false, DryRun flag is false
        let flags = DryRunAndCopyFlag {
            copy: false,
            dry_run: false,
        };
        let run_mode = get_run_mode_from_options(flags);
        match run_mode {
            RunMode::Normal => assert!(true),
            _ => assert!(false),
        }
    }
}
