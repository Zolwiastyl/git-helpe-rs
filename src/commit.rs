use core::panic;
use std::process::Command;

use anyhow::{Error, Result};

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

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(interpolated_commit)
        .output()
        .unwrap()
        .stdout;

    println!("{:?}", output);
    Ok(())
}
