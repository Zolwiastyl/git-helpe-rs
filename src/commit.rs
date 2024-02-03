use core::panic;
use std::process::Command;

use anyhow::{Error, Result};

use crate::cli::DryRunAndCopyFlag;
use crate::run_mode::get_run_mode_from_options;
use crate::run_mode::run_copy;
use crate::run_mode::RunMode;
use crate::template::interpolate_on_custom_val;
use crate::template::validate_interpolation_places_on_custom_pattern;
use crate::{
    cli::CommitOperationArguments,
    git_config::GitConfig,
    template::{interpolate, validate_interpolation_places_count},
};

pub fn commit_with_formatted_message(
    options: CommitOperationArguments,
    config: GitConfig,
) -> Result<(), Error> {
    let selected_commit_format = options.use_template.key;

    let _use_autocomplete_values = &options.use_template.use_autocomplete;
    let _auto_complete_values = &config.data.autocomplete_values;

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

    let interpolated_commit = if options.flags.use_branch_number {
        println!("{}", interpolated_commit);
        let higher_bound_of_digits_in_utf8 = &58;
        let lower_bound_of_digits_in_utf8 = &47;
        let branch_output = Command::new("git").arg("status").output().unwrap().stdout;
        let branch_number: Vec<u8> = branch_output
            .into_iter()
            .filter(|u8_char| {
                u8_char < higher_bound_of_digits_in_utf8 && u8_char > lower_bound_of_digits_in_utf8
            })
            .collect();
        let branch_number: String = String::from_utf8_lossy(&branch_number).to_string();
        let branch_number_as_interpolate_value = vec![branch_number];

        validate_interpolation_places_on_custom_pattern(
            &interpolated_commit,
            branch_number_as_interpolate_value.len(),
            "{b}",
        )
        .unwrap();

        interpolate_on_custom_val(
            &interpolated_commit,
            branch_number_as_interpolate_value,
            "{b}",
        )
        .unwrap()
    } else {
        interpolated_commit
    };

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
        RunMode::Copy => run_copy(
            &config,
            format!("git commit -m \"{}\"", interpolated_commit),
        ),
    };
}
