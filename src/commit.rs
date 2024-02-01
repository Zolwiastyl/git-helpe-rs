use core::panic;
use std::process::Command;
use std::process::Stdio;

use anyhow::{Error, Result};

use crate::cli::DryRunAndCopyFlag;
use crate::{
    cli::CommitOperationArguments,
    git_config::GitConfig,
    template::{interpolate, validate_interpolation_places_count},
};

enum RunMode {
    Normal,
    DryRun,
    DryRunAndCopy,
    Copy,
}

pub fn commit_with_formatted_message(
    options: CommitOperationArguments,
    config: GitConfig,
) -> Result<(), Error> {
    let selected_commit_format = options.use_template.key;

    let _use_autocomplete_values = options.use_template.use_autocomplete;
    let _auto_complete_values = config.data.autocomplete_values;

    let picked_commit_format = config
        .data
        .commit_template_variants
        .get(&selected_commit_format)
        .unwrap_or_else(|| {
            panic!(
                "No commit template under given key {} \n \
                You should add it prior to trying to use",
                selected_commit_format
            )
        });

    // TODO add {[]} autocomplete_values handling
    // TODO add {b} branch_values handling
    let is_valid = validate_interpolation_places_count(
        picked_commit_format,
        options.use_template.interpolate_values.len(),
    );

    if is_valid.is_err() {
        let err: Error = is_valid.err().unwrap();
        return Err(err);
    }

    let interpolated_commit = interpolate(
        picked_commit_format,
        options.use_template.interpolate_values,
    )?;

    // TODO here should be copy_flag checked
    // TODO here should be dry_run_flag checked
    let run_mode = get_run_mode_from_options(DryRunAndCopyFlag {
        dry_run: options.flags.dry_run,
        copy: options.flags.copy,
    });

    return match run_mode {
        RunMode::Normal => {
            let cmd = Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg(interpolated_commit)
                .output()
                .unwrap()
                .stdout;
            println!("{}", String::from_utf8_lossy(&cmd));

            Ok(())
        }
        RunMode::DryRun => {
            println!(
                "Going to run: \n \
        git commit -m \"{}\"",
                interpolated_commit
            );
            Ok(())
        }
        RunMode::DryRunAndCopy => {
            let copy_command = config.data.clipboard_commands.copy;
            println!(
                "Going to run: \n \
        echo 'git commit -m \"{}\"' > {}",
                interpolated_commit, copy_command
            );
            Ok(())
        }
        RunMode::Copy => {
            let copy_command = config.data.clipboard_commands.copy;
            let echo = Command::new("echo")
                .arg(format!("git commit -m \"{}\"", interpolated_commit))
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            Command::new(copy_command)
                .stdin(Stdio::from(echo.stdout.unwrap()))
                .output()
                .unwrap();

            Ok(())
        }
    };
}

fn get_run_mode_from_options(flags: DryRunAndCopyFlag) -> RunMode {
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
