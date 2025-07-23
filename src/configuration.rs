use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Configuration {
    pub base_dir: PathBuf,
}

impl Configuration {
    pub fn with_default_values(home_dir: &str) -> Self {
        let mut target_dir = PathBuf::from(home_dir);
        target_dir.push("src");

        Self {
            base_dir: target_dir,
        }
    }

    pub fn from_file(path: &PathBuf) -> Result<Self, String> {
        let Ok(config_str) = fs::read_to_string(path) else {
            return Err(String::from("Failed to read configuration file"));
        };

        match toml::from_str::<Configuration>(&config_str) {
            Ok(config) => Ok(config),
            Err(err) => Err(format!("Failed to parse configuration: {err}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

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

    #[rstest]
    fn test_from_file_succeeds_for_valid_files(#[files("test_fixtures/config_*.toml")] path: PathBuf) {
        let result = Configuration::from_file(&path);

        assert!(result.is_ok(), "from_file didn't return Ok: {result:?}");
    }

    #[rstest]
    fn test_from_file_fails_for_invalid_files(#[files("test_fixtures/invalid_config_*.toml")] path: PathBuf) {
        let result = Configuration::from_file(&path);

        assert!(result.is_err(), "from_file didn't return Err: {result:?}");
    }

    #[rstest]
    fn test_from_file_returns_expected_struct_for_valid_config() {
        let path = PathBuf::from("test_fixtures/config_base_dir.toml");
        let actual = Configuration::from_file(&path);
        let expected = Configuration { base_dir: PathBuf::from("/home/ferris/src") };

        assert_eq!(actual, Ok(expected));
    }
}
