use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Output};

fn write_out(output: &Output) -> Result<(), String> {
    match io::stdout().write_all(&output.stdout) {
        Ok(_) => (),
        Err(_) => return Err(String::from("error writing to stdout")),
    };

    match io::stderr().write_all(&output.stderr) {
        Ok(_) => (),
        Err(_) => return Err(String::from("error writing to stderr")),
    };

    Ok(())
}

pub fn can_access_remote(repo_str: &str) -> Result<bool, String> {
    let ls_remote_output = Command::new("git")
        .args(["ls-remote", "--branches"])
        .arg(repo_str)
        .output()
        .expect("error calling git ls-remote");

    if !ls_remote_output.status.success() {
        write_out(&ls_remote_output)?;
        return Ok(false);
    }

    Ok(true)
}

pub fn clone(repo_str: &str, target_dir: &Path) -> Result<(), String> {
    let clone_output = Command::new("git")
        .arg("clone")
        .arg(repo_str)
        .arg(target_dir)
        .status()
        .expect("error calling git clone");

    if clone_output.success() {
        Ok(())
    } else {
        Err(String::from("git clone returned non-zero status code"))
    }
}
