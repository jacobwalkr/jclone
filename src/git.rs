use std::path::Path;
use std::process::{Command, Stdio};

use crate::configuration::Configuration;
use crate::errors::JCloneError;
use crate::user_configuration::OutputStyle;

pub struct Git {
    git_executable: String,
    repo_str: String,
    print_progress: bool,
}

impl Git {
    pub fn new(repo_str: &str, config: &Configuration) -> Self {
        let print_progress = matches!(
            config.output_style,
            OutputStyle::Default | OutputStyle::GitOnly
        );

        Self {
            git_executable: config.git_executable.to_owned(),
            repo_str: repo_str.to_owned(),
            print_progress,
        }
    }

    fn ls_remote(&self) -> Result<(), JCloneError> {
        let stderr = match self.print_progress {
            true => Stdio::inherit(),
            false => Stdio::null(),
        };

        let ls_remote_output = Command::new(&self.git_executable)
            .args(["ls-remote", "--heads"])
            .arg(&self.repo_str)
            .stdout(Stdio::null())
            .stderr(stderr)
            .status();

        match ls_remote_output {
            Ok(output) if output.success() => Ok(()),
            Ok(_) => Err(JCloneError::GitUser {
                command: "ls-remote",
                message: "couldn't access remote",
            }),
            Err(err) => Err(JCloneError::GitSystem {
                executable: self.git_executable.to_owned(),
                command: "ls-remote",
                source: err,
            }),
        }
    }

    pub fn clone(&self, target_dir: &Path) -> Result<(), JCloneError> {
        self.ls_remote()?;

        let stdio = match self.print_progress {
            true => || Stdio::inherit(),
            false => || Stdio::null(),
        };

        let clone_status = Command::new(&self.git_executable)
            .arg("clone")
            .arg(&self.repo_str)
            .arg(target_dir)
            .stdout(stdio())
            .stderr(stdio())
            .status();

        match clone_status {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(JCloneError::GitUser {
                command: "clone",
                message: "returned non-zero exit code",
            }),
            Err(err) => Err(JCloneError::GitSystem {
                executable: self.git_executable.to_owned(),
                command: "clone",
                source: err,
            }),
        }
    }
}
