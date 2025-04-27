use serde::Deserialize;
use std::fs;

const DEFAULT_EXCLUDE_DIRS: &[&str] = &[
    "target",
    ".git",
    "node_modules",
    ".venv",
    "__pycache__",
    ".cache",
    "build",
    "target",
    "zig-out",
    ".zig-cache",
];
const DEFAULT_EXCLUDE_FILES: &[&str] = &["Cargo.lock", "poetry.lock", "package-lock.json"];

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_exclude_dirs")]
    pub exclude_dirs: Vec<String>,
    #[serde(default = "default_exclude_files")]
    pub exclude_files: Vec<String>,
}

fn default_exclude_dirs() -> Vec<String> {
    DEFAULT_EXCLUDE_DIRS.iter().map(|s| s.to_string()).collect()
}

fn default_exclude_files() -> Vec<String> {
    DEFAULT_EXCLUDE_FILES
        .iter()
        .map(|s| s.to_string())
        .collect()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            exclude_dirs: default_exclude_dirs(),
            exclude_files: default_exclude_files(),
        }
    }
}

pub fn load_config() -> Config {
    if let Ok(xdg_dirs) = xdg::BaseDirectories::new() {
        let config_home = xdg_dirs.get_config_home();
        let config_path = config_home.join("repocopy/config.yaml");

        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(content) => match serde_yaml::from_str(&content) {
                    Ok(config) => return config,
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to parse config file at {:?}: {}. Using defaults.",
                            config_path, e
                        );
                    }
                },
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to read config file at {:?}: {}. Using defaults.",
                        config_path, e
                    );
                }
            }
        }
    }
    Config::default()
}
