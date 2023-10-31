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
    pub commit_format: String,
    pub branch_format: String,
    pub branch_prefix_variants: HashMap<String, String>,
}

impl GitConfig {
    fn default_config() -> Self {
        return GitConfig {
            commit_format: "".to_string(),
            branch_format: "".to_string(),
            branch_prefix_variants: HashMap::new(),
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
            "There was no interpolation signature {{}} introduced in {name_of_field_to_check}"
        )));
    }
}
