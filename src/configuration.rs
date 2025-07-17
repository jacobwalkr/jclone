use std::path::PathBuf;

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
