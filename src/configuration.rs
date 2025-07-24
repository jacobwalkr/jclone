use std::{env, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct FileConfiguration {
    base_dir: Option<PathBuf>,
}

#[derive(Debug, PartialEq)]
pub struct Configuration {
    pub base_dir: PathBuf,
}

impl FileConfiguration {
    fn from_file(path: &PathBuf) -> Option<FileConfiguration> {
        if !path.exists() { return None };

        let Ok(config_str) = fs::read_to_string(path) else {
            println!("Failed to read config file");
            return None
        };

        match toml::from_str::<FileConfiguration>(&config_str) {
            Ok(config) => Some(config),
            Err(err) => {
                panic!("Failed to parse configuration: {err}");
            }
        }
    }
}

impl Configuration {
    fn with_default_values(home_dir: &str) -> Self {
        let mut target_dir = PathBuf::from(home_dir);
        target_dir.push("src");

        Self {
            base_dir: target_dir,
        }
    }

    pub fn load() -> Self {
        let home = env::var("HOME").expect("$HOME environment variable isn't set");
        let initial = Self::with_default_values(&home);

        let config_path = PathBuf::from(&home).join(".jclone.toml");

        FileConfiguration::from_file(&config_path)
            .and_then(|file_config| {
                Some(Self { base_dir: file_config.base_dir.unwrap_or(initial.base_dir.to_owned()) })
            })
            .unwrap_or(initial)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_default_values_generates_expected_defaults() {
        let home_dir = String::from("/some/directory");
        let mut base_dir = PathBuf::from(&home_dir);
        base_dir.push("src");

        let actual = Configuration::with_default_values(&home_dir);
        let expected = Configuration { base_dir };

        assert_eq!(actual, expected);
    }
}
