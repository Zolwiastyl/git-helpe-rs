use std::process::Command;

use anyhow::{anyhow, Result};
use regex::Regex;

use crate::{cli_arguments::BranchOperationArguments, git_config::GitConfig};

pub fn checkout_to_branch_with_prefix(
    options: BranchOperationArguments,
    config: GitConfig,
) -> Result<()> {
    println!("checking out to branch...");
    let checkout_regex = Regex::new(r"^git checkout -b [a-zA-Z0-9_.-]+$").unwrap();
    let clipboard_value = Command::new("pbpaste")
        // .arg("pbpaste")
        .output()
        .expect("Couldn't run pbpaste");
    println!("clipboard val: {:?}", clipboard_value.stdout);
    let output_as_string = String::from_utf8(clipboard_value.stdout).unwrap();
    // clipboard_value.
    // .expect("Couldn't convert resopnse from pbpaste to string");
    if !checkout_regex.is_match(&output_as_string) {
        println!("What you had in pb: {:?}", output_as_string);
        return Err(anyhow!(
            "What you have in your clipboard is not a valid git checkout command \n
        valid one looks like this: \n
        git checkout -b name-of-your-branch
        "
        ));
    }

    if let Some(prefix_found) = config
        .branch_prefix_variants
        .get(&options.branch_prefix_key)
    {
        let new_val = format!("{}{}", prefix_found, &output_as_string);
        println!("running command: {:?}", new_val);
        Command::new(new_val).output().unwrap();
        return Ok(());
    }
    return Err(anyhow!(
        "There was no prefix for key {} \n You should add it prior to trying to use",
        options.branch_prefix_key
    ));
}
