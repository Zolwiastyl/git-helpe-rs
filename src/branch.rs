use std::process::Command;

use anyhow::{anyhow, Result};
use regex::Regex;

use crate::{cli::CheckoutToPrefix, git_config::GitConfig};

pub fn checkout_to_branch_with_prefix(options: CheckoutToPrefix, config: GitConfig) -> Result<()> {
    let checkout_regex = Regex::new(r"^git checkout -b [a-zA-Z0-9_.-]+$").unwrap();
    let clipboard_value = Command::new("pbpaste")
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
