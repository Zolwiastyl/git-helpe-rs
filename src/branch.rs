use std::process::Command;

use anyhow::{anyhow, Error, Result};
use regex::Regex;

use crate::{
    cli::{CheckoutToPrefix, UseTemplate},
    git_config::GitConfig,
    template::{interpolate, validate_interpolation_places_count},
};

pub fn checkout_to_branch_with_prefix(options: CheckoutToPrefix, config: GitConfig) -> Result<()> {
    let checkout_regex = Regex::new(r"^git checkout -b [a-zA-Z0-9_.-]+$").unwrap();
    let paste_command = config.data.clipboard_commands.paste;

    let clipboard_value = Command::new(paste_command)
        .output()
        .expect("Couldn't run pbpaste");
    let output_as_string = String::from_utf8(clipboard_value.stdout).unwrap();

    if !checkout_regex.is_match(&output_as_string) {
        return Err(anyhow!(
            "What you have in your clipboard is not a valid git checkout command \n
        valid one looks like this: \n
        git checkout -b name-of-your-branch
        "
        ));
    }

    if let Some(prefix_found) = config.data.branch_prefix_variants.get(&options.prefix_key) {
        let split_on_space: Vec<String> =
            output_as_string.split(" ").map(|s| s.to_string()).collect();

        let after_prefix = &split_on_space[3..].join("");

        let new_val = prefix_found.to_owned() + after_prefix;

        let result = Command::new("git")
            .arg("checkout")
            .arg("-b")
            .arg(new_val)
            .output()
            .unwrap();

        println!("git output: \n {:?}", String::from_utf8(result.stdout));
        return Ok(());
    }
    return Err(anyhow!(
        "There was no prefix for key {} \n You should add it prior to trying to use",
        options.prefix_key
    ));
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

    // TODO here should be copy_flag checked
    // TODO here should be dry_run flag checked

    let output = Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(interpolated_branch)
        .output()
        .unwrap()
        .stdout;

    println!("{:?}", output);
    Ok(())
}
