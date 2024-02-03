use std::process::Command;

use anyhow::{anyhow, Error, Result};
use regex::Regex;

use crate::{
    cli::{CheckoutToPrefix, DryRunAndCopyFlag, UseTemplate},
    git_config::GitConfig,
    run_mode::{get_run_mode_from_options, run_copy, RunMode},
    template::{interpolate, validate_interpolation_places_count},
};

pub fn checkout_to_branch_with_prefix(options: CheckoutToPrefix, config: GitConfig) -> Result<()> {
    let checkout_regex = Regex::new(r"^git checkout -b [a-zA-Z0-9_.-]+$").unwrap();
    let paste_command = &config.data.clipboard_commands.paste;

    let clipboard_value = Command::new(paste_command)
        .output()
        .expect("Couldn't run paste_command");

    let output_as_string = String::from_utf8(clipboard_value.stdout).unwrap();

    if !checkout_regex.is_match(&output_as_string) {
        return Err(anyhow!(
            "What you have in your clipboard is not a valid git checkout command \n
        valid one looks like this: \n
        git checkout -b name-of-your-branch
        "
        ));
    }
    let prefix_found = match config.data.branch_prefix_variants.get(&options.prefix_key) {
        None => {
            return Err(anyhow!(
                "There was no prefix for key {} \n You should add it prior to trying to use",
                options.prefix_key
            ))
        }
        Some(prefix) => prefix,
    };

    let split_on_space: Vec<String> = output_as_string.split(" ").map(|s| s.to_string()).collect();

    let after_prefix = &split_on_space[3..].join("");

    let full_branch_name = prefix_found.to_owned() + after_prefix;

    let run_mode = get_run_mode_from_options(DryRunAndCopyFlag {
        dry_run: options.dry_run,

        copy: options.copy,
    });

    return match run_mode {
        RunMode::Normal => {
            let result = Command::new("git")
                .arg("checkout")
                .arg("-b")
                .arg(full_branch_name)
                .output()
                .unwrap();

            println!("git output: \n {:?}", String::from_utf8(result.stdout));
            Ok(())
        }
        RunMode::DryRun => {
            println!(
                "Going to run: \n \
                    git checkout -b {}",
                full_branch_name
            );
            Ok(())
        }
        RunMode::Copy => run_copy(&config, format!("git checkout -b {}", full_branch_name)),
        RunMode::DryRunAndCopy => {
            let copy_command = config.data.clipboard_commands.copy;

            println!(
                "Going to run: \n \
        echo 'git checkout -b {}' > {}",
                full_branch_name, copy_command
            );
            Ok(())
        }
    };
}

pub fn checkout_to_branch_with_template(
    options: UseTemplate,
    config: GitConfig,
) -> Result<(), Error> {
    let selected_branch_format = options.key;

    let picked_branch_format = config
        .data
        .branch_template_variants
        .get(&selected_branch_format)
        .unwrap_or_else(|| {
            panic!(
                "No branch template under given key {} \n \
       You should add it prior to trying to use 
        ",
                selected_branch_format
            )
        });

    let is_valid =
        validate_interpolation_places_count(picked_branch_format, options.interpolate_values.len());

    if is_valid.is_err() {
        let err: Error = is_valid.err().unwrap();
        return Err(err);
    };

    let interpolate_values = options.interpolate_values;
    // Had to figure out around closure that couldn't move value
    let interpolate_values_for_debugging = format!("{:?}", interpolate_values);
    let interpolated_branch =
        interpolate(picked_branch_format, interpolate_values).unwrap_or_else(|_err| {
            panic!(
                "Couldn't interpolate branch format \n \
                Trying to interpolate template: \n \
                {} \n \
                with: \n \
                {:?}
                ",
                picked_branch_format, interpolate_values_for_debugging
            )
        });

    let run_mode = get_run_mode_from_options(DryRunAndCopyFlag {
        dry_run: options.dry_run,
        copy: options.copy,
    });

    match run_mode {
        RunMode::Normal => {
            let output = Command::new("git")
                .arg("checkout")
                .arg("-b")
                .arg(interpolated_branch)
                .output()
                .unwrap()
                .stdout;

            println!("{}", String::from_utf8_lossy(&output));
            Ok(())
        }
        RunMode::DryRun => {
            let command_to_print = format!("git checkout -b {}", interpolated_branch);
            println!("Command to be executed: \n {}", command_to_print);
            Ok(())
        }
        RunMode::DryRunAndCopy => {
            let copy_command = config.data.clipboard_commands.copy;

            let command_to_print = format!(
                "echo 'git checkout -b {}' > {}",
                interpolated_branch, copy_command
            );
            let message_to_print =
                format!("command that's going to be run: \n {}", command_to_print);
            println!("{}", message_to_print);
            Ok(())
        }
        RunMode::Copy => {
            let value_to_copy = format!("git checkout -b {}", interpolated_branch);
            run_copy(&config, value_to_copy)
        }
    }
}
