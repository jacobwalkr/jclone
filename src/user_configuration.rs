use serde::Deserialize;
use std::{env, fs, path::PathBuf};

use crate::errors::{JCloneError, JCloneResult};

#[derive(Deserialize, Debug, PartialEq, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum OutputStyle {
    #[default]
    Default,
    GitOnly,
    NoGit,
    Quiet,
}

#[derive(Deserialize, Debug, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct UserConfiguration {
    pub base_dir: Option<String>,
    pub use_host_dir: Option<bool>,
    pub use_full_path: Option<bool>,
    #[serde(rename = "output")]
    pub output_style: Option<OutputStyle>,
    pub git_executable: Option<String>,
    #[serde(default)]
    #[serde(rename = "variant")]
    pub variants: Vec<UserHostConfiguration>,
}

#[derive(Deserialize, Debug, PartialEq, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct UserHostConfiguration {
    pub host: String,
    pub base_dir: Option<String>,
    pub use_host_dir: Option<bool>,
    pub use_full_path: Option<bool>,
    #[serde(rename = "output")]
    pub output_style: Option<OutputStyle>,
    pub git_executable: Option<String>,
}

impl TryFrom<String> for UserConfiguration {
    type Error = toml::de::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        toml::from_str(&value)
    }
}

impl UserConfiguration {
    pub fn variant_matching_host(&self, host: &String) -> UserHostConfiguration {
        self.variants
            .iter()
            .find(|variant| variant.host == *host)
            .map_or_else(UserHostConfiguration::default, |v| v.to_owned())
    }

    pub fn try_load() -> JCloneResult<Self> {
        let home = PathBuf::from(env::var("HOME").map_err(JCloneError::Environment)?);
        let config_path = PathBuf::from(&home).join(".jclone.toml");

        if !config_path.exists() {
            return Ok(Self::default());
        }

        match fs::read_to_string(&config_path) {
            Ok(config_str) => Ok(toml::from_str(&config_str)
                .map_err(|err| JCloneError::ConfigurationParse(config_path, err))?),
            Err(err) => Err(JCloneError::ConfigurationFileLoad(config_path, err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_configuration_load_can_deserialize_empty_file() {
        let user_config_str = String::from("");
        let actual = UserConfiguration::try_from(user_config_str).unwrap();

        let default_config = UserConfiguration {
            base_dir: None,
            use_host_dir: None,
            use_full_path: None,
            output_style: None,
            git_executable: None,
            variants: vec![],
        };

        assert_eq!(actual, default_config);
    }

    #[test]
    fn test_user_configuration_load_can_deserialize_user_configuration_without_variants() {
        let user_config_str = String::from(
            r#"
            base_dir = "/base/dir"
            use_host_dir = false
            use_full_path = true
            output = "no-git"
            git_executable = "/home/ferris/bin/git"
            "#,
        );

        let expected = UserConfiguration {
            base_dir: Some(String::from("/base/dir")),
            use_host_dir: Some(false),
            use_full_path: Some(true),
            output_style: Some(OutputStyle::NoGit),
            git_executable: Some(String::from("/home/ferris/bin/git")),
            variants: vec![],
        };

        let actual = UserConfiguration::try_from(user_config_str).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_user_configuration_load_can_deserialize_complete_user_configuration() {
        let user_config_str = String::from(
            r#"
            base_dir = "/base/dir"
            use_host_dir = false
            use_full_path = true
            output = "git-only"
            git_executable = "/home/ferris/bin/git"

            [[variant]]
            host = "example.com"
            base_dir = "/second/dir"
            use_host_dir = true
            use_full_path = true
            output = "default"
            git_executable = "bin/git-example-com"

            [[variant]]
            host = "example.net"
            base_dir = "/third/dir"
            use_host_dir = false
            use_full_path = false
            output = "quiet"
            git_executable = "bin/git-example-net"
            "#,
        );

        let expected = UserConfiguration {
            base_dir: Some(String::from("/base/dir")),
            use_host_dir: Some(false),
            use_full_path: Some(true),
            output_style: Some(OutputStyle::GitOnly),
            git_executable: Some(String::from("/home/ferris/bin/git")),
            variants: vec![
                UserHostConfiguration {
                    host: String::from("example.com"),
                    base_dir: Some(String::from("/second/dir")),
                    use_host_dir: Some(true),
                    use_full_path: Some(true),
                    output_style: Some(OutputStyle::Default),
                    git_executable: Some(String::from("bin/git-example-com")),
                },
                UserHostConfiguration {
                    host: String::from("example.net"),
                    base_dir: Some(String::from("/third/dir")),
                    use_host_dir: Some(false),
                    use_full_path: Some(false),
                    output_style: Some(OutputStyle::Quiet),
                    git_executable: Some(String::from("bin/git-example-net")),
                },
            ],
        };

        let actual = UserConfiguration::try_from(user_config_str).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn test_user_configuration_load_panics_denies_unknown_fields_at_root() {
        let user_config_str = String::from(
            r#"
            unexpected = "I shouldn't be here"
            "#,
        );

        let _ = UserConfiguration::try_from(user_config_str).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_user_configuration_load_panics_denies_unknown_fields_in_variants() {
        let user_config_str = String::from(
            r#"
            [[variant]]
            unexpected = "I shouldn't be here"
            "#,
        );

        let _ = UserConfiguration::try_from(user_config_str).unwrap();
    }

    #[test]
    fn test_variant_matching_host_when_one_matching() {
        let host = String::from("example.com");

        let user_config = UserConfiguration {
            variants: vec![UserHostConfiguration {
                host: String::from("example.com"),
                base_dir: Some(String::from("/some/other/directory")),
                ..UserHostConfiguration::default()
            }],
            ..UserConfiguration::default()
        };

        let actual = user_config.variant_matching_host(&host);

        let expected = UserHostConfiguration {
            host: String::from("example.com"),
            base_dir: Some(String::from("/some/other/directory")),
            ..UserHostConfiguration::default()
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_variant_matching_host_when_none_matching() {
        let host = String::from("no-match.example.net");

        let user_config = UserConfiguration {
            variants: vec![UserHostConfiguration {
                host: String::from("example.com"),
                base_dir: Some(String::from("/some/other/directory")),
                ..UserHostConfiguration::default()
            }],
            ..UserConfiguration::default()
        };

        let actual = user_config.variant_matching_host(&host);
        let expected = UserHostConfiguration::default();

        assert_eq!(actual, expected);
    }
}
