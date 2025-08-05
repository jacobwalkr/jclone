use std::{
    env,
    path::{Path, PathBuf},
};

use crate::user_configuration::UserConfiguration;

#[derive(Debug, PartialEq)]
pub struct Configuration {
    pub base_dir: String,
    pub use_host_dir: bool,
    pub use_full_path: bool,
}

impl Configuration {
    fn from_user_configuration(user_config: UserConfiguration, host: &String, home: &Path) -> Self {
        let host_variant = user_config.variant_matching_host(host);

        Self {
            base_dir: host_variant
                .base_dir
                .or(user_config.base_dir)
                .unwrap_or_else(|| home.join("src").to_str().unwrap().to_owned()),
            use_host_dir: host_variant
                .use_host_dir
                .or(user_config.use_host_dir)
                .unwrap_or(true),
            use_full_path: host_variant
                .use_full_path
                .or(user_config.use_full_path)
                .unwrap_or(true),
        }
    }

    pub fn load(host: &String) -> Self {
        let user_config = UserConfiguration::load().unwrap_or_default();
        let home = PathBuf::from(env::var("HOME").expect("$HOME environment variable isn't set"));

        Self::from_user_configuration(user_config, host, &home)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use rstest::{fixture, rstest};

    use crate::user_configuration::{UserConfiguration, UserHostConfiguration};

    use super::*;

    #[fixture]
    fn base_user_config() -> UserConfiguration {
        UserConfiguration {
            base_dir: Some(String::from("/some/other/directory")),
            use_host_dir: Some(false),
            use_full_path: Some(false),
            variants: Default::default(),
        }
    }

    #[fixture]
    fn complete_user_config() -> UserConfiguration {
        UserConfiguration {
            variants: vec![
                UserHostConfiguration {
                    host: String::from("example.org"),
                    base_dir: Some(String::from("/dir/example-org")),
                    ..UserHostConfiguration::default()
                },
                UserHostConfiguration {
                    host: String::from("example.com"),
                    base_dir: Some(String::from("/dir/example-com")),
                    use_host_dir: Some(true),
                    use_full_path: Some(true),
                },
            ],
            ..base_user_config()
        }
    }

    #[test]
    fn test_from_user_configuration_empty_user_config() {
        let default_user_config = UserConfiguration::default();
        let host = String::from("no-match.example.com");
        let home = PathBuf::from("/some/directory");

        let actual = Configuration::from_user_configuration(default_user_config, &host, &home);
        let expected = Configuration {
            base_dir: String::from("/some/directory/src"),
            use_host_dir: true,
            use_full_path: true,
        };

        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_from_user_configuration_base_user_config(base_user_config: UserConfiguration) {
        let host = String::from("no-match.example.net");
        let home = PathBuf::from("/some/directory");
        let actual = Configuration::from_user_configuration(base_user_config, &host, &home);

        let expected = Configuration {
            base_dir: String::from("/some/other/directory"),
            use_host_dir: false,
            use_full_path: false,
        };

        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_from_user_configuration_non_matching_host(complete_user_config: UserConfiguration) {
        let host = String::from("no-match.example.net");
        let home = PathBuf::from("/some/directory");
        let actual = Configuration::from_user_configuration(complete_user_config, &host, &home);

        let expected = Configuration {
            base_dir: String::from("/some/other/directory"),
            use_host_dir: false,
            use_full_path: false,
        };

        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_from_user_configuration_matching_host(complete_user_config: UserConfiguration) {
        let host = String::from("example.com");
        let home = PathBuf::from("/some/directory");
        let actual = Configuration::from_user_configuration(complete_user_config, &host, &home);

        let expected = Configuration {
            base_dir: String::from("/dir/example-com"),
            use_host_dir: true,
            use_full_path: true,
        };

        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_from_user_configuration_partial_config_matching_host_inherits_all_levels(
        complete_user_config: UserConfiguration,
    ) {
        let partial_user_config = UserConfiguration {
            use_full_path: None,
            ..complete_user_config
        };

        let host = String::from("example.org");
        let home_dir = PathBuf::from("/some/directory");
        let actual = Configuration::from_user_configuration(partial_user_config, &host, &home_dir);

        let expected = Configuration {
            base_dir: String::from("/dir/example-org"),
            use_host_dir: false,
            use_full_path: true,
        };

        assert_eq!(actual, expected);
    }
}
