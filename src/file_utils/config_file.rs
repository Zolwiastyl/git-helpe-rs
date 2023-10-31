use std::path::PathBuf;

pub fn get_path_to_config(path: Option<PathBuf>) -> PathBuf {
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
