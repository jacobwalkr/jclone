use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Output};

fn write_out(output: &Output) -> Result<(), String> {
    match io::stdout().write_all(&output.stdout) {
        Ok(_) => (),
        Err(err) => return Err(format!("error writing to stdout: {err}")),
    };

    match io::stderr().write_all(&output.stderr) {
        Ok(_) => (),
        Err(err) => return Err(format!("error writing to stderr: {err}")),
    };

    Ok(())
}

pub fn can_access_remote(repo_str: &str) -> Result<(), String> {
    let ls_remote_output = Command::new("git")
        .args(["ls-remote", "--branches"])
        .arg(repo_str)
        .output();

    match ls_remote_output {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => match write_out(&output) {
            Ok(_) => Err(String::from("couldn't access repository")),
            Err(err) => Err(format!("couldn't access repository, and encountered {err}")),
        },
        Err(err) => Err(format!("error calling git ls-remote: {err}")),
    }
}

pub fn clone(repo_str: &str, target_dir: &Path) -> Result<(), String> {
    let clone_status = Command::new("git")
        .arg("clone")
        .arg(repo_str)
        .arg(target_dir)
        .status();

    match clone_status {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err(String::from("git clone returned non-zero status code")),
        Err(err) => Err(format!("error calling git clone: {err}")),
    }
}
