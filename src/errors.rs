use std::{env::VarError, io, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JCloneError {
    #[error("Error: {0}")]
    Generic(&'static str),

    #[error("Error: {0}")]
    Environment(#[from] VarError),

    #[error("Couldn't load configuration from {0}: {1}")]
    ConfigurationFileLoad(PathBuf, #[source] io::Error),

    #[error("Couldn't parse configuration from: {0}:\n{1}")]
    ConfigurationParse(PathBuf, toml::de::Error),

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
