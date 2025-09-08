use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JCloneError {
    #[error("{executable} {command}: {source}")]
    GitSystem {
        executable: String,
        command: &'static str,
        source: io::Error,
    },

    #[error("git {command}: {message}")]
    GitUser {
        command: &'static str,
        message: &'static str,
    },

    #[error("Couldn't parse repository: {0}")]
    RepositoryParse(&'static str),
}

pub type JCloneResult<T> = Result<T, JCloneError>;
