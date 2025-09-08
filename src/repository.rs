use crate::errors::{JCloneError, JCloneResult};

#[derive(Debug, PartialEq)]
pub struct Repository {
    pub host: String,
    pub path: String,
}

impl TryFrom<&String> for Repository {
    type Error = JCloneError;

    fn try_from(repo_str: &String) -> JCloneResult<Self> {
        let part_after_proto = match repo_str.split_once("://") {
            None => repo_str,
            Some((_, p)) => p,
        };

        let (prefix, suffix) = match part_after_proto.split_once(':') {
            Some(("", _)) | Some((_, "")) | None => {
                return Err(JCloneError::RepositoryParse("unexpected format"));
            }
            Some((prefix, _)) if prefix.contains('/') => {
                return Err(JCloneError::RepositoryParse(
                    "local paths are not supported",
                ));
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
    use rstest::rstest;

    #[test]
    fn test_empty_string_error() {
        let repo_str = String::from("");

        assert!(Repository::try_from(&repo_str).is_err());
    }

    #[rstest]
    #[case::ssh_compact("example.com:my_repo", "example.com", "my_repo")]
    #[case::ssh_user("git@example.com:my_repo", "example.com", "my_repo")]
    #[case::ssh_not_git_user("someone-else@example.com:my_repo", "example.com", "my_repo")]
    #[case::ssh_scheme("ssh://git@example.com:my_repo", "example.com", "my_repo")]
    #[case::ssh_scheme_extension("ssh://git@example.com:my_repo.git", "example.com", "my_repo")]
    #[case::ssh_scheme_extension_long_path(
        "ssh://git@example.com:my_user/my_group/my_repo.git",
        "example.com",
        "my_user/my_group/my_repo"
    )]
    #[case::ssh_scheme_extension_path_with_starting_slash(
        "ssh://git@example.com:/my_user/my_repo.git",
        "example.com",
        "my_user/my_repo"
    )]
    #[case::git_scheme_extension("git://git@example.com:my_repo.git", "example.com", "my_repo")]
    fn test_repo_string_correctly_converted_to_repository(
        #[case] input: String,
        #[case] host: String,
        #[case] path: String,
    ) -> Result<(), JCloneError> {
        let expected = Repository { host, path };

        assert_eq!(Repository::try_from(&input)?, expected);
        Ok(())
    }
}
