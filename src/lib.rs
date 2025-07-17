use std::io::{self, Write};
use std::{fs, path::PathBuf, process::Command};

pub fn git_clone(repo_str: &str, target_dir: &PathBuf) -> Result<(), String> {
    fs::create_dir_all(target_dir).expect("error creating clone directory");

    let command_output = Command::new("git")
        .arg("clone")
        .arg("--quiet")
        .arg(repo_str)
        .arg(target_dir)
        .output()
        .expect("error calling git");

    match io::stdout().write_all(&command_output.stdout) {
        Ok(_) => (),
        Err(_) => return Err(String::from("error writing to stdout")),
    };

    match io::stderr().write_all(&command_output.stderr) {
        Ok(_) => (),
        Err(_) => return Err(String::from("error writing to stderr")),
    };

    if command_output.status.success() {
        Ok(())
    } else {
        Err(String::from("git returned non-zero status code"))
    }
}
