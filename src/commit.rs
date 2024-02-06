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
    let _autocomplete_values = &config.data.autocomplete_values;

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
        let branch_output = Command::new("git").arg("status").output().unwrap().stdout;

        let branch_number = get_branch_number_from_git_status_output(branch_output).unwrap();

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
        println!("no flag");
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

fn get_branch_number_from_git_status_output(git_status_output: Vec<u8>) -> Result<String, Error> {
    let higher_bound_of_digits_in_utf8 = &58;
    let lower_bound_of_digits_in_utf8 = &47;
    let line_with_branch_name: Vec<u8> = String::from_utf8(git_status_output)
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>()[0]
        .into();

    let branch_number: Vec<u8> = line_with_branch_name
        .into_iter()
        .filter(|u8_char| {
            u8_char < higher_bound_of_digits_in_utf8 && u8_char > lower_bound_of_digits_in_utf8
        })
        .collect();
    let branch_number: String = String::from_utf8_lossy(&branch_number).to_string();
    if branch_number == "" {
        return Err(Error::msg("There is no number in branch name"));
    }

    Ok(branch_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_branch_number_from_git_status_output_when_no_numbers_in_branch() {
        let git_output: Vec<u8> = vec![
            79, 110, 32, 98, 114, 97, 110, 99, 104, 32, 109, 97, 105, 110, 10, 89, 111, 117, 114,
            32, 98, 114, 97, 110, 99, 104, 32, 105, 115, 32, 117, 112, 32, 116, 111, 32, 100, 97,
            116, 101, 32, 119, 105, 116, 104, 32, 39, 111, 114, 105, 103, 105, 110, 47, 109, 97,
            105, 110, 39, 46, 10, 10, 67, 104, 97, 110, 103, 101, 115, 32, 110, 111, 116, 32, 115,
            116, 97, 103, 101, 100, 32, 102, 111, 114, 32, 99, 111, 109, 109, 105, 116, 58, 10, 32,
            32, 40, 117, 115, 101, 32, 34, 103, 105, 116, 32, 97, 100, 100, 32, 60, 102, 105, 108,
            101, 62, 46, 46, 46, 34, 32, 116, 111, 32, 117, 112, 100, 97, 116, 101, 32, 119, 104,
            97, 116, 32, 119, 105, 108, 108, 32, 98, 101, 32, 99, 111, 109, 109, 105, 116, 116,
            101, 100, 41, 10, 32, 32, 40, 117, 115, 101, 32, 34, 103, 105, 116, 32, 114, 101, 115,
            116, 111, 114, 101, 32, 60, 102, 105, 108, 101, 62, 46, 46, 46, 34, 32, 116, 111, 32,
            100, 105, 115, 99, 97, 114, 100, 32, 99, 104, 97, 110, 103, 101, 115, 32, 105, 110, 32,
            119, 111, 114, 107, 105, 110, 103, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 41,
            10, 9, 109, 111, 100, 105, 102, 105, 101, 100, 58, 32, 32, 32, 115, 114, 99, 47, 98,
            114, 97, 110, 99, 104, 46, 114, 115, 10, 9, 109, 111, 100, 105, 102, 105, 101, 100, 58,
            32, 32, 32, 115, 114, 99, 47, 99, 111, 109, 109, 105, 116, 46, 114, 115, 10, 9, 109,
            111, 100, 105, 102, 105, 101, 100, 58, 32, 32, 32, 115, 114, 99, 47, 114, 117, 110, 95,
            109, 111, 100, 101, 46, 114, 115, 10, 10, 110, 111, 32, 99, 104, 97, 110, 103, 101,
            115, 32, 97, 100, 100, 101, 100, 32, 116, 111, 32, 99, 111, 109, 109, 105, 116, 32, 40,
            117, 115, 101, 32, 34, 103, 105, 116, 32, 97, 100, 100, 34, 32, 97, 110, 100, 47, 111,
            114, 32, 34, 103, 105, 116, 32, 99, 111, 109, 109, 105, 116, 32, 45, 97, 34, 41, 10,
        ];

        let output = get_branch_number_from_git_status_output(git_output);
        match output {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
    #[test]
    fn get_branch_number_from_git_status_output_when_no_numbers_beside_branch() {
        let output: Vec<u8> = vec![
            79, 110, 32, 98, 114, 97, 110, 99, 104, 32, 102, 101, 97, 116, 117, 114, 101, 47, 49,
            50, 51, 52, 47, 115, 111, 109, 101, 116, 104, 105, 110, 103, 45, 100, 105, 102, 102,
            101, 114, 101, 110, 116, 10, 67, 104, 97, 110, 103, 101, 115, 32, 110, 111, 116, 32,
            115, 116, 97, 103, 101, 100, 32, 102, 111, 114, 32, 99, 111, 109, 109, 105, 116, 58,
            10, 32, 32, 40, 117, 115, 101, 32, 34, 103, 105, 116, 32, 97, 100, 100, 32, 60, 102,
            105, 108, 101, 62, 46, 46, 46, 34, 32, 116, 111, 32, 117, 112, 100, 97, 116, 101, 32,
            119, 104, 97, 116, 32, 119, 105, 108, 108, 32, 98, 101, 32, 99, 111, 109, 109, 105,
            116, 116, 101, 100, 41, 10, 32, 32, 40, 117, 115, 101, 32, 34, 103, 105, 116, 32, 114,
            101, 115, 116, 111, 114, 101, 32, 60, 102, 105, 108, 101, 62, 46, 46, 46, 34, 32, 116,
            111, 32, 100, 105, 115, 99, 97, 114, 100, 32, 99, 104, 97, 110, 103, 101, 115, 32, 105,
            110, 32, 119, 111, 114, 107, 105, 110, 103, 32, 100, 105, 114, 101, 99, 116, 111, 114,
            121, 41, 10, 9, 109, 111, 100, 105, 102, 105, 101, 100, 58, 32, 32, 32, 115, 114, 99,
            47, 98, 114, 97, 110, 99, 104, 46, 114, 115, 10, 9, 109, 111, 100, 105, 102, 105, 101,
            100, 58, 32, 32, 32, 115, 114, 99, 47, 99, 111, 109, 109, 105, 116, 46, 114, 115, 10,
            9, 109, 111, 100, 105, 102, 105, 101, 100, 58, 32, 32, 32, 115, 114, 99, 47, 114, 117,
            110, 95, 109, 111, 100, 101, 46, 114, 115, 10, 10, 110, 111, 32, 99, 104, 97, 110, 103,
            101, 115, 32, 97, 100, 100, 101, 100, 32, 116, 111, 32, 99, 111, 109, 109, 105, 116,
            32, 40, 117, 115, 101, 32, 34, 103, 105, 116, 32, 97, 100, 100, 34, 32, 97, 110, 100,
            47, 111, 114, 32, 34, 103, 105, 116, 32, 99, 111, 109, 109, 105, 116, 32, 45, 97, 34,
            41, 10,
        ];
        let output = get_branch_number_from_git_status_output(output).unwrap();
        assert_eq!(output, "1234");
    }
    #[test]
    fn get_branch_number_from_git_status_output_when_numbers_in_files_in_status() {
        let output: Vec<u8> = vec![
            79, 110, 32, 98, 114, 97, 110, 99, 104, 32, 102, 101, 97, 116, 117, 114, 101, 47, 49,
            50, 51, 52, 47, 115, 111, 109, 101, 116, 104, 105, 110, 103, 45, 100, 105, 102, 102,
            101, 114, 101, 110, 116, 10, 67, 104, 97, 110, 103, 101, 115, 32, 110, 111, 116, 32,
            115, 116, 97, 103, 101, 100, 32, 102, 111, 114, 32, 99, 111, 109, 109, 105, 116, 58,
            10, 32, 32, 40, 117, 115, 101, 32, 34, 103, 105, 116, 32, 97, 100, 100, 32, 60, 102,
            105, 108, 101, 62, 46, 46, 46, 34, 32, 116, 111, 32, 117, 112, 100, 97, 116, 101, 32,
            119, 104, 97, 116, 32, 119, 105, 108, 108, 32, 98, 101, 32, 99, 111, 109, 109, 105,
            116, 116, 101, 100, 41, 10, 32, 32, 40, 117, 115, 101, 32, 34, 103, 105, 116, 32, 114,
            101, 115, 116, 111, 114, 101, 32, 60, 102, 105, 108, 101, 62, 46, 46, 46, 34, 32, 116,
            111, 32, 100, 105, 115, 99, 97, 114, 100, 32, 99, 104, 97, 110, 103, 101, 115, 32, 105,
            110, 32, 119, 111, 114, 107, 105, 110, 103, 32, 100, 105, 114, 101, 99, 116, 111, 114,
            121, 41, 10, 9, 109, 111, 100, 105, 102, 105, 101, 100, 58, 32, 32, 32, 115, 114, 99,
            47, 98, 114, 97, 110, 99, 104, 46, 114, 115, 10, 9, 109, 111, 100, 105, 102, 105, 101,
            100, 58, 32, 32, 32, 115, 114, 99, 47, 99, 111, 109, 109, 105, 116, 46, 114, 115, 10,
            9, 109, 111, 100, 105, 102, 105, 101, 100, 58, 32, 32, 32, 115, 114, 99, 47, 114, 117,
            110, 95, 109, 111, 100, 101, 46, 114, 115, 10, 10, 85, 110, 116, 114, 97, 99, 107, 101,
            100, 32, 102, 105, 108, 101, 115, 58, 10, 32, 32, 40, 117, 115, 101, 32, 34, 103, 105,
            116, 32, 97, 100, 100, 32, 60, 102, 105, 108, 101, 62, 46, 46, 46, 34, 32, 116, 111,
            32, 105, 110, 99, 108, 117, 100, 101, 32, 105, 110, 32, 119, 104, 97, 116, 32, 119,
            105, 108, 108, 32, 98, 101, 32, 99, 111, 109, 109, 105, 116, 116, 101, 100, 41, 10, 9,
            115, 111, 109, 101, 95, 102, 105, 108, 101, 95, 49, 50, 51, 46, 116, 120, 116, 10, 9,
            115, 111, 109, 101, 95, 102, 105, 108, 101, 95, 55, 56, 57, 46, 116, 120, 116, 10, 10,
            110, 111, 32, 99, 104, 97, 110, 103, 101, 115, 32, 97, 100, 100, 101, 100, 32, 116,
            111, 32, 99, 111, 109, 109, 105, 116, 32, 40, 117, 115, 101, 32, 34, 103, 105, 116, 32,
            97, 100, 100, 34, 32, 97, 110, 100, 47, 111, 114, 32, 34, 103, 105, 116, 32, 99, 111,
            109, 109, 105, 116, 32, 45, 97, 34, 41, 10,
        ];
        let output = get_branch_number_from_git_status_output(output).unwrap();
        assert_eq!(output, "1234")
    }
    #[test]
    fn get_branch_number_from_git_status_output_when_numbers_only_in_files_in_status() {
        let output: Vec<u8> = vec![
            79, 110, 32, 98, 114, 97, 110, 99, 104, 32, 109, 97, 105, 110, 10, 89, 111, 117, 114,
            32, 98, 114, 97, 110, 99, 104, 32, 105, 115, 32, 117, 112, 32, 116, 111, 32, 100, 97,
            116, 101, 32, 119, 105, 116, 104, 32, 39, 111, 114, 105, 103, 105, 110, 47, 109, 97,
            105, 110, 39, 46, 10, 10, 67, 104, 97, 110, 103, 101, 115, 32, 110, 111, 116, 32, 115,
            116, 97, 103, 101, 100, 32, 102, 111, 114, 32, 99, 111, 109, 109, 105, 116, 58, 10, 32,
            32, 40, 117, 115, 101, 32, 34, 103, 105, 116, 32, 97, 100, 100, 32, 60, 102, 105, 108,
            101, 62, 46, 46, 46, 34, 32, 116, 111, 32, 117, 112, 100, 97, 116, 101, 32, 119, 104,
            97, 116, 32, 119, 105, 108, 108, 32, 98, 101, 32, 99, 111, 109, 109, 105, 116, 116,
            101, 100, 41, 10, 32, 32, 40, 117, 115, 101, 32, 34, 103, 105, 116, 32, 114, 101, 115,
            116, 111, 114, 101, 32, 60, 102, 105, 108, 101, 62, 46, 46, 46, 34, 32, 116, 111, 32,
            100, 105, 115, 99, 97, 114, 100, 32, 99, 104, 97, 110, 103, 101, 115, 32, 105, 110, 32,
            119, 111, 114, 107, 105, 110, 103, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 41,
            10, 9, 109, 111, 100, 105, 102, 105, 101, 100, 58, 32, 32, 32, 115, 114, 99, 47, 98,
            114, 97, 110, 99, 104, 46, 114, 115, 10, 9, 109, 111, 100, 105, 102, 105, 101, 100, 58,
            32, 32, 32, 115, 114, 99, 47, 99, 111, 109, 109, 105, 116, 46, 114, 115, 10, 9, 109,
            111, 100, 105, 102, 105, 101, 100, 58, 32, 32, 32, 115, 114, 99, 47, 114, 117, 110, 95,
            109, 111, 100, 101, 46, 114, 115, 10, 10, 85, 110, 116, 114, 97, 99, 107, 101, 100, 32,
            102, 105, 108, 101, 115, 58, 10, 32, 32, 40, 117, 115, 101, 32, 34, 103, 105, 116, 32,
            97, 100, 100, 32, 60, 102, 105, 108, 101, 62, 46, 46, 46, 34, 32, 116, 111, 32, 105,
            110, 99, 108, 117, 100, 101, 32, 105, 110, 32, 119, 104, 97, 116, 32, 119, 105, 108,
            108, 32, 98, 101, 32, 99, 111, 109, 109, 105, 116, 116, 101, 100, 41, 10, 9, 115, 111,
            109, 101, 95, 102, 105, 108, 101, 95, 49, 50, 51, 46, 116, 120, 116, 10, 9, 115, 111,
            109, 101, 95, 102, 105, 108, 101, 95, 55, 56, 57, 46, 116, 120, 116, 10, 10, 110, 111,
            32, 99, 104, 97, 110, 103, 101, 115, 32, 97, 100, 100, 101, 100, 32, 116, 111, 32, 99,
            111, 109, 109, 105, 116, 32, 40, 117, 115, 101, 32, 34, 103, 105, 116, 32, 97, 100,
            100, 34, 32, 97, 110, 100, 47, 111, 114, 32, 34, 103, 105, 116, 32, 99, 111, 109, 109,
            105, 116, 32, 45, 97, 34, 41, 10,
        ];
        let output = get_branch_number_from_git_status_output(output);
        match output {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
}
