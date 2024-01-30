use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    cli::{CommitOperationArguments, SetFormat, UseTemplate},
    file_utils::config_file::get_path_to_config,
};
use anyhow::{Error, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GitConfig {
    pub data: Data,
    config_path: PathBuf,
}

type Variants = HashMap<String, String>;
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub commit_template_variants: Variants,
    pub branch_template_variants: Variants,
    pub branch_prefix_variants: Variants,
}

pub struct Formats {
    pub commit_format_variants: Variants,
    pub branch_format_variants: Variants,
}

pub enum BranchOrCommitAction {
    Commit(CommitOperationArguments),
    BranchFromTemplate(UseTemplate),
}

// impl TryInto<SetFormat> for BranchOrCommitAction {
//     type Error = Error;
//     fn try_into(self) -> std::result::Result<SetFormat, Self::Error> {
//         match self {
//             BranchOrCommitAction::BranchFromTemplate(args) => Ok(SetFormat {
//                 key: args.key,
//                 value: args.interpolate_values,
//             }),
//             BranchOrCommitAction::Commit(args) => Ok(SetFormat {
//                 key: args.use_template.key,
//                 value: args.use_template.interpolate_values,
//             }),
//         }
//     }
// }

impl Data {
    fn default() -> Self {
        Data {
            commit_template_variants: HashMap::new(),
            branch_template_variants: HashMap::new(),
            branch_prefix_variants: HashMap::new(),
        }
    }
}

impl GitConfig {
    fn default_config() -> Self {
        return GitConfig {
            data: Data::default(),
            config_path: get_path_to_config(None).to_path_buf(),
        };
    }

    pub fn new_config(
        branch_prefix_variants: Variants,
        branch_format_variants: Variants,
        commit_format_variants: Variants,
        config_path: Option<PathBuf>,
    ) -> Self {
        return GitConfig {
            data: Data {
                branch_template_variants: branch_format_variants,
                commit_template_variants: commit_format_variants,
                branch_prefix_variants,
            },
            config_path: if let Some(config_path) = config_path {
                config_path
            } else {
                get_path_to_config(None).to_path_buf()
            },
        };
    }

    pub fn from_file(path_to_file: PathBuf) -> Self {
        if fs::metadata(&path_to_file).is_ok() {
            let contents = fs::read_to_string(&path_to_file);
            let contents = contents
                .unwrap_or("{\"commit_format\": \"\", \"branch_format\": \"\"}".to_string());

            let data = serde_json::from_str(&contents);
            let data: Data = data.unwrap_or(Data::default());

            GitConfig {
                data: data,
                config_path: path_to_file,
            }
        } else {
            Self::default_config()
        }
    }

    pub fn validate_against_interpolation_regex<'a>(
        string_to_interpolate: &'a String,
        name_of_field_to_check: &'static str,
    ) -> Result<&'a String> {
        let interpolation_regex = Regex::new(r"\{.*?\}").unwrap();
        if interpolation_regex.is_match(string_to_interpolate) {
            return Ok(string_to_interpolate);
        };
        return Err(Error::msg(format!(
            "There was no interpolation signature: {{}} introduced in {name_of_field_to_check}"
        )));
    }

    // TODO
    // This should get a key as well
    pub fn set_format(&mut self, args: BranchOrCommitAction) -> Result<()> {
        let new_formats: Formats = match args {
            BranchOrCommitAction::BranchFromTemplate(action_args) => {
                let result =
                    Self::validate_against_interpolation_regex(&action_args.value, "branch_format");
                match result {
                    Err(err) => panic!("{}", err),
                    Ok(new_val) => Formats {
                        branch_format: new_val.to_string(),
                        commit_format: self.data.commit_format.to_owned(),
                    },
                }
            }
            BranchOrCommitAction::BranchFromTemplate(action_args) => {
                let result =
                    Self::validate_against_interpolation_regex(&action_args.value, "branch_format");
                match result {
                    Err(err) => panic!("{}", err),
                    Ok(new_val) => Formats {
                        branch_format: self.data.branch_format.to_owned(),
                        commit_format: new_val.to_string(),
                    },
                }
            }
        };

        // FIXME
        // This looks like an obvious overwrite
        self.data.branch_format = new_formats.branch_format;
        self.data.commit_format = new_formats.commit_format;

        self.save_to_file()?;
        Ok(())
    }
    pub fn set_commit_template_from_variatn(&mut self, args: SetFormat) -> Result<()> {
        let commit_variants_map = self.data.commit_template_variants;
        todo!("implement")
    }

    pub fn set_branch_template_form_variant(&mut self, args: SetFormat) -> Result<()> {
        let branch_variants_map = self.data.branch_template_variants;
        let result = Self::validate_against_interpolation_regex(&args.value, &args.key);
        todo!("implement")
    }

    pub fn set_branch_prefix_variant(&mut self, key: String, value: String) -> Result<()> {
        self.data.branch_prefix_variants.insert(key, value);
        self.save_to_file()?;
        Ok(())
    }

    pub fn delete_branch_prefix_variant(&mut self, key: String) -> Result<()> {
        let old_val = self.data.branch_prefix_variants.remove(&key);
        println!(
            "Removed {} : {} from config ",
            key,
            old_val.unwrap_or(String::from("None"))
        );
        self.save_to_file()?;
        Ok(())
    }

    fn save_to_file(&self) -> Result<()> {
        if let Some(dir) = self.config_path.parent() {
            if !std::fs::metadata(&dir).is_ok() {
                std::fs::create_dir_all(dir)?;
            }
        };

        let contents = serde_json::to_string(&self.data)?;
        std::fs::write(&self.config_path, contents)?;
        return Ok(());
    }

    pub fn display_config(&self) -> Result<String> {
        let branch = self.data.branch_template_variants.to_owned();
        let commit = self.data.commit_template_variants.to_owned();
        let prefixes = self.data.branch_prefix_variants.to_owned();

        Ok(format!(
            "
        branch format: {:?} 
        commit format: {:?} 
        branch prefixes: {:?} 
        ",
            branch, commit, prefixes
        ))
    }
}
