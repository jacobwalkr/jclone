use std::path::Path;
use std::process::{Command, Stdio};

pub fn can_access_remote(
    repo_str: &str,
    inherit_stderr: bool,
    report_git_errors: bool,
) -> Result<bool, String> {
    let stderr = match inherit_stderr {
        true => Stdio::inherit(),
        false => Stdio::null(),
    };

    let ls_remote_output = Command::new("git")
        .args(["ls-remote", "--branches"])
        .arg(repo_str)
        .stdout(Stdio::null())
        .stderr(stderr)
        .status();

    match ls_remote_output {
        Ok(output) if output.success() => Ok(true),
        Ok(_) if report_git_errors => Err(String::from("couldn't access repository")),
        Ok(_) => Ok(false),
        Err(err) => Err(format!("error calling git ls-remote: {err}")),
    }
}

pub fn clone(
    repo_str: &str,
    target_dir: &Path,
    print_progress: bool,
    report_git_errors: bool,
) -> Result<(), String> {
    let stdio = match print_progress {
        true => || Stdio::inherit(),
        false => || Stdio::null(),
    };

    let clone_status = Command::new("git")
        .arg("clone")
        .arg(repo_str)
        .arg(target_dir)
        .stdout(stdio())
        .stderr(stdio())
        .status();

    match clone_status {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => match report_git_errors {
            true => Err(String::from("git clone returned non-zero status code")),
            false => Ok(()),
        },
        Err(err) => Err(format!("error calling git clone: {err}")),
    }
}
