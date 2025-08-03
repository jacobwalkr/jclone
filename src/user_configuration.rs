use std::{env, fs, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct UserConfiguration {
    pub base_dir: Option<String>,
    pub use_host_dir: Option<bool>,
    pub use_full_path: Option<bool>,
}

impl From<String> for UserConfiguration {
    fn from(value: String) -> Self {
        toml::from_str(&value).unwrap_or_else(|err| panic!("Failed to parse configuration: {err}"))
    }
}

impl UserConfiguration {
    pub fn load() -> Option<Self> {
        let home = PathBuf::from(env::var("HOME").expect("$HOME environment variable isn't set"));
        let config_path = PathBuf::from(&home).join(".jclone.toml");

        config_path
            .exists()
            .then(|| {
                fs::read_to_string(&config_path)
                    .unwrap_or_else(|err| panic!("Error reading config file: {err}"))
            })
            .map(Self::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_configuration_load_can_deserialize_complete_user_configuration() {
        let user_config_str = String::from(
            r#"
            base_dir = "/base/dir"
            use_host_dir = false
            use_full_path = true
            "#,
        );

        let expected = UserConfiguration {
            base_dir: Some(String::from("/base/dir")),
            use_host_dir: Some(false),
            use_full_path: Some(true),
        };

        let actual = UserConfiguration::from(user_config_str);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_user_configuration_load_can_deserialize_empty_file() {
        let user_config_str = String::from("");
        let actual = UserConfiguration::from(user_config_str);

        let default_config = UserConfiguration {
            base_dir: None,
            use_host_dir: None,
            use_full_path: None,
        };

        assert_eq!(actual, default_config);
    }

    #[test]
    #[should_panic]
    fn test_user_configuration_load_panics_denies_unknown_fields_at_root() {
        let user_config_str = String::from(
            r#"
            unexpected = "I shouldn't be here"
            "#,
        );

        let _ = UserConfiguration::from(user_config_str);
    }
}
