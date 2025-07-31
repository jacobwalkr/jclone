use std::{
    env, fs,
    path::{Path, PathBuf},
};

use toml::Table;

#[derive(Debug, PartialEq)]
pub struct Configuration {
    pub base_dir: PathBuf,
    pub use_host_dir: bool,
    pub use_full_path: bool,
}

impl Configuration {
    fn with_default_values(user_values: Table, home: &Path) -> Self {
        Self {
            base_dir: user_values
                .get("base_dir")
                .and_then(|value| value.as_str())
                .map(PathBuf::from)
                .unwrap_or_else(|| home.join("src")),
            use_host_dir: user_values
                .get("use_host_dir")
                .and_then(|value| value.as_bool())
                .unwrap_or(true),
            use_full_path: user_values
                .get("use_full_path")
                .and_then(|value| value.as_bool())
                .unwrap_or(true),
        }
    }

    pub fn load() -> Self {
        let home = PathBuf::from(env::var("HOME").expect("$HOME environment variable isn't set"));
        let config_path = PathBuf::from(&home).join(".jclone.toml");

        config_path
            .exists()
            .then(|| {
                fs::read_to_string(&config_path)
                    .unwrap_or_else(|err| panic!("Error reading config file: {err}"))
            })
            .map(|config_str| {
                config_str
                    .parse::<Table>()
                    .unwrap_or_else(|err| panic!("Failed to parse configuration: {err}"))
            })
            .map(|config_table| Self::with_default_values(config_table, &home))
            .unwrap_or_else(|| Self::with_default_values(Table::new(), &home))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_default_values_with_empty_table_generates_expected_defaults() {
        let empty_table = Table::new();
        let home_dir = PathBuf::from("/some/directory");

        let actual = Configuration::with_default_values(empty_table, &home_dir);
        let expected = Configuration {
            base_dir: home_dir.join("src"),
            use_host_dir: true,
            use_full_path: true,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_with_default_values_with_complete_table_generates_expected_configuration() {
        let table = r#"
            base_dir = "/some/other/directory"
            use_host_dir = false
            use_full_path = false
        "#
        .parse::<Table>()
        .unwrap();
        let home_dir = PathBuf::from("/some/directory");

        let actual = Configuration::with_default_values(table, &home_dir);
        let expected = Configuration {
            base_dir: PathBuf::from("/some/other/directory"),
            use_host_dir: false,
            use_full_path: false,
        };

        assert_eq!(actual, expected);
    }
}
