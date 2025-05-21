#[derive(Debug, PartialEq)]
pub struct Repository {
    pub host: String,
    pub path: String,
}

impl TryFrom<&String> for Repository {
    type Error = String;

    fn try_from(repo_str: &String) -> Result<Self, Self::Error> {
        let part_after_proto = match repo_str.split_once("://") {
            None => repo_str,
            Some((_, p)) => p,
        };

        let (prefix, suffix) = match part_after_proto.split_once(':') {
            Some(("", _)) | Some((_, "")) | None => return Err("unexpected format".to_owned()),
            Some((prefix, _)) if prefix.contains('/') => {
                return Err("looks like local path".to_owned());
            }
            Some(parts) => parts,
        };

        let host_part = match prefix.split_once('@') {
            None => prefix,
            Some((_, host)) => host,
        };

        let path_part_trimmed = suffix.trim_start_matches('/');

        let path_part = match path_part_trimmed.strip_suffix(".git") {
            None => path_part_trimmed,
            Some(p) => p,
        };

        Ok(Repository {
            host: host_part.to_owned(),
            path: path_part.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string_error() {
        let repo_str = String::from("");

        assert!(Repository::try_from(&repo_str).is_err());
    }

    #[test]
    fn test_ssh_compact() -> Result<(), String> {
        let repo_str = String::from("example.com:my_repo");
        let expected = Repository {
            host: String::from("example.com"),
            path: String::from("my_repo"),
        };

        assert_eq!(Repository::try_from(&repo_str)?, expected);
        Ok(())
    }

    #[test]
    fn test_ssh_user() -> Result<(), String> {
        let repo_str = String::from("git@example.com:my_repo");
        let expected = Repository {
            host: String::from("example.com"),
            path: String::from("my_repo"),
        };

        assert_eq!(Repository::try_from(&repo_str)?, expected);
        Ok(())
    }

    #[test]
    fn test_ssh_not_git_user() -> Result<(), String> {
        let repo_str = String::from("someone-else@example.com:my_repo");
        let expected = Repository {
            host: String::from("example.com"),
            path: String::from("my_repo"),
        };

        assert_eq!(Repository::try_from(&repo_str)?, expected);
        Ok(())
    }

    #[test]
    fn test_ssh_scheme() -> Result<(), String> {
        let repo_str = String::from("ssh://git@example.com:my_repo");
        let expected = Repository {
            host: String::from("example.com"),
            path: String::from("my_repo"),
        };

        assert_eq!(Repository::try_from(&repo_str)?, expected);
        Ok(())
    }

    #[test]
    fn test_ssh_scheme_extension() -> Result<(), String> {
        let repo_str = String::from("ssh://git@example.com:my_repo.git");
        let expected = Repository {
            host: String::from("example.com"),
            path: String::from("my_repo"),
        };

        assert_eq!(Repository::try_from(&repo_str)?, expected);
        Ok(())
    }

    #[test]
    fn test_ssh_scheme_extension_long_path() -> Result<(), String> {
        let repo_str = String::from("ssh://git@example.com:my_user/my_group/my_repo.git");
        let expected = Repository {
            host: String::from("example.com"),
            path: String::from("my_user/my_group/my_repo"),
        };

        assert_eq!(Repository::try_from(&repo_str)?, expected);
        Ok(())
    }

    #[test]
    fn test_ssh_scheme_extension_path_with_starting_slash() -> Result<(), String> {
        let repo_str = String::from("ssh://git@example.com:/my_user/my_repo.git");
        let expected = Repository {
            host: String::from("example.com"),
            path: String::from("my_user/my_repo"),
        };

        assert_eq!(Repository::try_from(&repo_str)?, expected);
        Ok(())
    }

    #[test]
    fn test_git_scheme_extension() -> Result<(), String> {
        let repo_str = String::from("git://git@example.com:my_repo.git");
        let expected = Repository {
            host: String::from("example.com"),
            path: String::from("my_repo"),
        };

        assert_eq!(Repository::try_from(&repo_str)?, expected);
        Ok(())
    }
}
