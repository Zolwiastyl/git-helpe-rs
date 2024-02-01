use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    cli::{CommitOperationArguments, SetClipboardCommands, SetFormat, UseTemplate},
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
pub struct ClipboardCommands {
    pub copy: String,
    pub paste: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub clipboard_commands: ClipboardCommands,
    pub commit_template_variants: Variants,
    pub branch_template_variants: Variants,
    pub branch_prefix_variants: Variants,
    pub autocomplete_values: Option<Vec<String>>,
}

pub struct Templates {
    pub commit_template_variants: Variants,
    pub branch_template_variants: Variants,
}

pub enum BranchOrCommitAction {
    Commit(CommitOperationArguments),
    BranchFromTemplate(UseTemplate),
}

impl Data {
    fn default() -> Self {
        Data {
            clipboard_commands: ClipboardCommands {
                copy: "pbcopy".to_string(),
                paste: "pbpaste".to_string(),
            },
            commit_template_variants: HashMap::new(),
            branch_template_variants: HashMap::new(),
            branch_prefix_variants: HashMap::new(),
            autocomplete_values: None,
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
        clipboard_commands: ClipboardCommands,
        branch_prefix_variants: Variants,
        branch_format_variants: Variants,
        commit_format_variants: Variants,
        config_path: Option<PathBuf>,
    ) -> Self {
        return GitConfig {
            data: Data {
                clipboard_commands,
                branch_template_variants: branch_format_variants,
                commit_template_variants: commit_format_variants,
                branch_prefix_variants,
                autocomplete_values: None,
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
            let contents = contents.unwrap_or(
                "{\
                    \"clipboard_commands\": {}, \
                    \"branch_template_variants\": {}, \
                    \"commit_template_variants\": {}, \
                    \"branch_prefix_variants\": {}
                }"
                .to_string(),
            );

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

    pub fn set_branch_template_variant(&mut self, arg: SetFormat) -> Result<()> {
        let result = Self::validate_against_interpolation_regex(&arg.value, "branch_template");
        match result {
            Err(e) => panic!("{}", e),
            Ok(_) => self
                .data
                .branch_template_variants
                .insert(arg.key, arg.value),
        };
        self.save_to_file()
    }

    pub fn set_commit_template_variant(&mut self, arg: SetFormat) -> Result<()> {
        let result = Self::validate_against_interpolation_regex(&arg.value, "commit_template");
        match result {
            Err(e) => panic!("{}", e),
            Ok(_) => {
                self.data
                    .commit_template_variants
                    .insert(arg.key, arg.value);
            }
        };
        self.save_to_file()
    }

    pub fn set_branch_prefix_variant(&mut self, key: String, value: String) -> Result<()> {
        self.data.branch_prefix_variants.insert(key, value);
        self.save_to_file()
    }

    pub fn set_clipboard_command(&mut self, args: SetClipboardCommands) -> Result<()> {
        let new_clipboard_commands = ClipboardCommands {
            copy: args.copy,
            paste: args.paste,
        };

        self.data.clipboard_commands = new_clipboard_commands;
        self.save_to_file()
    }

    pub fn delete_branch_prefix_variant(&mut self, key: String) -> Result<()> {
        let old_val = self.data.branch_prefix_variants.remove(&key);
        println!(
            "Removed {} : {} from config ",
            key,
            old_val.unwrap_or(String::from("None"))
        );
        self.save_to_file()
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
        let clipboard_command = &self.data.clipboard_commands;
        let copy = &clipboard_command.copy;
        let paste = &clipboard_command.paste;
        let branch = self.data.branch_template_variants.to_owned();
        let commit = self.data.commit_template_variants.to_owned();
        let prefixes = self.data.branch_prefix_variants.to_owned();

        Ok(format!(
            "
        clipboard commands: {{
            \"copy\": {:?}
            \"paste\": {:?}
        }}
        branch formats: {:?} 
        commit formats: {:?} 
        branch prefixes: {:?} 
        ",
            *copy, *paste, branch, commit, prefixes
        ))
    }
}
