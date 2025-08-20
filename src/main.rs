use std::env;
use std::fs;
use std::path::Component;
use std::path::PathBuf;

use crate::configuration::Configuration;
use crate::repository::Repository;

mod configuration;
mod git;
mod repository;
mod user_configuration;

fn main() {
    let arg_repo = env::args().nth(1).expect("expecting argument: repository");

    match jclone(arg_repo) {
        Ok(_) => println!("🎉 Done!"),
        Err(err) => println!("❌ Error: {err}"),
    }
}

pub fn jclone(repo_str: String) -> Result<(), String> {
    let repository = Repository::try_from(&repo_str).expect("couldn't parse repository");
    let config = Configuration::load(&repository.host);
    let target_dir = target_dir(&repository, &config);

    match git::can_access_remote(&repo_str) {
        Ok(true) => fs::create_dir_all(&target_dir).expect("error creating clone directory"),
        Ok(false) => return Err(String::from("can't access repository")),
        Err(err) => return Err(err),
    }

    git::clone(&repo_str, &target_dir)?;

    Ok(())
}

fn target_dir(repo: &Repository, config: &Configuration) -> PathBuf {
    let mut dir = PathBuf::from(&config.base_dir);

    if config.use_host_dir {
        dir.push(&repo.host)
    }

    let repo_path = PathBuf::from(&repo.path);
    let mut components = repo_path.components().peekable();

    while let Some(component) = components.next() {
        match component {
            Component::Prefix(_) | Component::RootDir => (),
            _ if !config.use_full_path && components.peek().is_some() => (),
            _ => dir.push(component),
        };
    }

    dir
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn default_repo() -> Repository {
        Repository {
            host: String::from("example.com"),
            path: String::from("some/big/long/path"),
        }
    }

    #[fixture]
    fn default_config() -> Configuration {
        Configuration {
            base_dir: String::from("/home/ferris/src"),
            use_host_dir: true,
            use_full_path: true,
        }
    }

    #[rstest]
    fn test_target_dir_returns_whole_path_given_default_config(
        default_repo: Repository,
        default_config: Configuration,
    ) {
        assert_eq!(
            target_dir(&default_repo, &default_config),
            PathBuf::from("/home/ferris/src/example.com/some/big/long/path")
        )
    }

    #[rstest]
    fn test_target_dir_skips_host_dir_when_use_host_dir_is_false(
        default_repo: Repository,
        default_config: Configuration,
    ) {
        let config = Configuration {
            use_host_dir: false,
            ..default_config
        };

        assert_eq!(
            target_dir(&default_repo, &config),
            PathBuf::from("/home/ferris/src/some/big/long/path")
        )
    }

    #[rstest]
    fn test_target_dir_ensures_repo_path_is_relative(
        default_repo: Repository,
        default_config: Configuration,
    ) {
        let repo = Repository {
            path: String::from("/some/absolute/path"),
            ..default_repo
        };

        assert_eq!(
            target_dir(&repo, &default_config),
            PathBuf::from("/home/ferris/src/example.com/some/absolute/path")
        )
    }

    #[rstest]
    fn test_target_dir_uses_only_project_name_when_use_full_path_is_false(
        default_repo: Repository,
        default_config: Configuration,
    ) {
        let config = Configuration {
            use_full_path: false,
            ..default_config
        };

        assert_eq!(
            target_dir(&default_repo, &config),
            PathBuf::from("/home/ferris/src/example.com/path")
        )
    }
}
