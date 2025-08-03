use std::{
    env,
    path::{Path, PathBuf},
};

use crate::user_configuration::UserConfiguration;

#[derive(Debug, PartialEq)]
pub struct Configuration {
    pub base_dir: PathBuf,
    pub use_host_dir: bool,
    pub use_full_path: bool,
}

impl Configuration {
    fn from_user_configuration(user_config: UserConfiguration, home: &Path) -> Self {
        Self {
            base_dir: user_config.base_dir.unwrap_or_else(|| home.join("src")),
            use_host_dir: user_config.use_host_dir.unwrap_or(true),
            use_full_path: user_config.use_full_path.unwrap_or(true),
        }
    }

    pub fn load() -> Self {
        let user_config = UserConfiguration::load().unwrap_or_default();
        let home = PathBuf::from(env::var("HOME").expect("$HOME environment variable isn't set"));

        Self::from_user_configuration(user_config, &home)
    }
}

#[cfg(test)]
mod tests {
    use crate::user_configuration::UserConfiguration;

    use super::*;

    #[test]
    fn test_from_user_configuration_with_default_user_config_generates_expected_defaults() {
        let default_user_config = UserConfiguration::default();
        let home_dir = PathBuf::from("/some/directory");

        let actual = Configuration::from_user_configuration(default_user_config, &home_dir);
        let expected = Configuration {
            base_dir: home_dir.join("src"),
            use_host_dir: true,
            use_full_path: true,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_user_configuration_with_complete_user_config_generates_expected_configuration() {
        let user_config = UserConfiguration {
            base_dir: Some(PathBuf::from("/some/other/directory")),
            use_host_dir: Some(false),
            use_full_path: Some(false),
        };

        let home_dir = PathBuf::from("/some/directory");
        let actual = Configuration::from_user_configuration(user_config, &home_dir);

        let expected = Configuration {
            base_dir: PathBuf::from("/some/other/directory"),
            use_host_dir: false,
            use_full_path: false,
        };

        assert_eq!(actual, expected);
    }
}
