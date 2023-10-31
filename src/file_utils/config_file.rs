use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

struct ConfigFile {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitConfig {
    commit_format: String,
    branch_format: String,
}

impl GitConfig {
    fn default_config() -> Self {
        return GitConfig {
            commit_format: "".to_string(),
            branch_format: "".to_string(),
        };
    }
    fn from_file(path_to_file: PathBuf) -> Self {
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
}

pub fn open(custom_path_to_config_file: Option<PathBuf>) -> GitConfig {
    let path_to_config = get_path_to_config(custom_path_to_config_file);
    GitConfig::from_file(path_to_config)
}

fn get_path_to_config(path: Option<PathBuf>) -> PathBuf {
    if let Some(path) = path {
        return path;
    }

    let mut home = if let Ok(home) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(home)
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home)
    } else {
        panic!("Couldn't find home directory")
    };
    home.push(".git-helpe-rs-config");
    return home;
}
