use std::{env, path::PathBuf};

use crate::{
    errors::{JCloneError, JCloneResult},
    user_configuration::{OutputStyle, UserConfiguration},
};

#[derive(Debug, PartialEq)]
pub struct Configuration {
    pub base_dir: String,
    pub use_host_dir: bool,
    pub use_full_path: bool,
    pub output_style: OutputStyle,
    pub git_executable: String,
}

fn default_base_dir(home: String) -> JCloneResult<String> {
    Ok(PathBuf::from(home)
        .join("src")
        .to_str()
        .ok_or(JCloneError::Generic(String::from(
            "UTF-8 error parsing HOME directory",
        )))?
        .to_owned())
}

impl Configuration {
    fn from_user_configuration(
        user_config: UserConfiguration,
        host: &String,
        home: String,
    ) -> JCloneResult<Self> {
        let host_variant = user_config.variant_matching_host(host);

        let base_dir = match host_variant.base_dir.or(user_config.base_dir) {
            Some(dir) => dir,
            None => default_base_dir(home)?,
        };

        Ok(Self {
            base_dir,
            use_host_dir: host_variant
                .use_host_dir
                .or(user_config.use_host_dir)
                .unwrap_or(true),
            use_full_path: host_variant
                .use_full_path
                .or(user_config.use_full_path)
                .unwrap_or(true),
            output_style: host_variant
                .output_style
                .or(user_config.output_style)
                .unwrap_or_default(),
            git_executable: host_variant
                .git_executable
                .or(user_config.git_executable)
                .unwrap_or(String::from("git")),
        })
    }

    pub fn try_load(host: &String) -> JCloneResult<Self> {
        let user_config = UserConfiguration::try_load()?;
        let home_str = env::var("HOME").map_err(JCloneError::Environment)?;

        Self::from_user_configuration(user_config, host, home_str)
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
            output_style: Some(OutputStyle::Quiet),
            git_executable: Some(String::from("/home/ferris/bin/git")),
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
                    output_style: Some(OutputStyle::GitOnly),
                    git_executable: Some(String::from("bin/git-example-com")),
                },
            ],
            ..base_user_config()
        }
    }

    #[test]
    fn test_from_user_configuration_empty_user_config() {
        let default_user_config = UserConfiguration::default();
        let host = String::from("no-match.example.com");
        let home = String::from("/some/directory");

        let actual =
            Configuration::from_user_configuration(default_user_config, &host, home).unwrap();
        let expected = Configuration {
            base_dir: String::from("/some/directory/src"),
            use_host_dir: true,
            use_full_path: true,
            output_style: OutputStyle::Default,
            git_executable: String::from("git"),
        };

        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_from_user_configuration_base_user_config(base_user_config: UserConfiguration) {
        let host = String::from("no-match.example.net");
        let home = String::from("/some/directory");
        let actual = Configuration::from_user_configuration(base_user_config, &host, home).unwrap();

        let expected = Configuration {
            base_dir: String::from("/some/other/directory"),
            use_host_dir: false,
            use_full_path: false,
            output_style: OutputStyle::Quiet,
            git_executable: String::from("/home/ferris/bin/git"),
        };

        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_from_user_configuration_non_matching_host(complete_user_config: UserConfiguration) {
        let host = String::from("no-match.example.net");
        let home = String::from("/some/directory");
        let actual =
            Configuration::from_user_configuration(complete_user_config, &host, home).unwrap();

        let expected = Configuration {
            base_dir: String::from("/some/other/directory"),
            use_host_dir: false,
            use_full_path: false,
            output_style: OutputStyle::Quiet,
            git_executable: String::from("/home/ferris/bin/git"),
        };

        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_from_user_configuration_matching_host(complete_user_config: UserConfiguration) {
        let host = String::from("example.com");
        let home = String::from("/some/directory");
        let actual =
            Configuration::from_user_configuration(complete_user_config, &host, home).unwrap();

        let expected = Configuration {
            base_dir: String::from("/dir/example-com"),
            use_host_dir: true,
            use_full_path: true,
            output_style: OutputStyle::GitOnly,
            git_executable: String::from("bin/git-example-com"),
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
        let home = String::from("/some/directory");
        let actual =
            Configuration::from_user_configuration(partial_user_config, &host, home).unwrap();

        let expected = Configuration {
            base_dir: String::from("/dir/example-org"),
            use_host_dir: false,
            use_full_path: true,
            output_style: OutputStyle::Quiet,
            git_executable: String::from("/home/ferris/bin/git"),
        };

        assert_eq!(actual, expected);
    }
}
