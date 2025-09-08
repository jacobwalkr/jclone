use std::path::Path;
use std::process::{Command, Stdio};

use crate::configuration::Configuration;
use crate::user_configuration::OutputStyle;

pub struct Git {
    git_executable: String,
    repo_str: String,
    print_progress: bool,
    report_errors: bool,
}

impl Git {
    pub fn new(repo_str: &str, config: &Configuration) -> Self {
        let report_errors = matches!(
            config.output_style,
            OutputStyle::Default | OutputStyle::NoGit
        );

        let print_progress = matches!(
            config.output_style,
            OutputStyle::Default | OutputStyle::GitOnly
        );

        Self {
            git_executable: config.git_executable.to_owned(),
            repo_str: repo_str.to_owned(),
            print_progress,
            report_errors,
        }
    }

    fn can_access_remote(&self) -> Result<bool, String> {
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
            Ok(output) if output.success() => Ok(true),
            Ok(_) if self.report_errors => Err(String::from("couldn't access repository")),
            Ok(_) => Ok(false),
            Err(err) => Err(format!(
                "error calling `{0} ls-remote`: {err}",
                &self.git_executable
            )),
        }
    }

    pub fn clone(&self, target_dir: &Path) -> Result<(), String> {
        match self.can_access_remote() {
            Ok(true) => (),
            Ok(false) => return Ok(()),
            Err(err) => return Err(err),
        }

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
            Ok(_) => match self.report_errors {
                true => Err(String::from("git clone returned non-zero status code")),
                false => Ok(()),
            },
            Err(err) => Err(format!(
                "error calling `{0} clone`: {err}",
                &self.git_executable
            )),
        }
    }
}
