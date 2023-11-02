use std::{collections::HashMap, fs, path::PathBuf};

use crate::file_utils::config_file::get_path_to_config;
use anyhow::{Error, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};

pub fn open(custom_path_to_config_file: Option<PathBuf>) -> GitConfig {
    let path_to_config = get_path_to_config(custom_path_to_config_file);
    GitConfig::from_file(path_to_config)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitConfig {
    data: Data,
    config_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub commit_format: String,
    pub branch_format: String,
    pub branch_prefix_variants: HashMap<String, String>,
}

pub struct Formats {
    pub commit_format: String,
    pub branch_format: String,
}

impl GitConfig {
    fn default_config() -> Self {
        return GitConfig {
            data: Data {
                commit_format: "".to_string(),
                branch_format: "".to_string(),
                branch_prefix_variants: HashMap::new(),
            },
            config_path: get_path_to_config(None),
        };
    }

    pub fn from_file(path_to_file: PathBuf) -> Self {
        if fs::metadata(&path_to_file).is_ok() {
            let contents = fs::read_to_string(&path_to_file);
            let contents = contents
                .unwrap_or("{\"commit_format\": \"\", \"branch_format\": \"\"}".to_string());
            let data = serde_json::from_str(&contents);
            data.unwrap_or(Self::default_config())
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

    pub fn set_format(&mut self, key: &str, value: String) -> Result<()> {
        let new_formats: Formats = match key {
            "branch_format" => {
                let result = Self::validate_against_interpolation_regex(&value, "branch_format");
                match result {
                    Err(err) => panic!("{}", err),
                    Ok(new_val) => Formats {
                        branch_format: new_val.to_string(),
                        commit_format: self.data.commit_format.to_owned(),
                    },
                }
            }
            "commit_format" => {
                let result = Self::validate_against_interpolation_regex(&value, "branch_format");
                match result {
                    Err(err) => panic!("{}", err),
                    Ok(new_val) => Formats {
                        branch_format: self.data.branch_format.to_owned(),
                        commit_format: new_val.to_string(),
                    },
                }
            }
            _ => {
                return Err(Error::msg(format!(
                "Invalid key {} was passed, allowed keys are 'branch_format' and 'branch_commit'",
                key
            )))
            }
        };

        self.data.branch_format = new_formats.branch_format;
        self.data.commit_format = new_formats.commit_format;

        self.save_to_file()?;
        Ok(())
    }

    fn set_branch_prefix_variants(&mut self, key: &str, value: String) -> Result<()> {
        self.data
            .branch_prefix_variants
            .insert(String::from(key), value);
        self.save_to_file()?;
        Ok(())
    }

    fn delete_branch_prefix_variants(&mut self, key: &str) -> Result<()> {
        let old_val = self.data.branch_prefix_variants.remove(&String::from(key));
        println!("Removed {} : {:?} from config ", key, old_val);
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
}
