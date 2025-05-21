mod repository;

use repository::Repository;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

fn main() {
    let arg_repo = env::args().nth(1).expect("expecting argument: repository");
    let repository = Repository::try_from(&arg_repo).expect("couldn't parse repository");

    let home_dir = env::var("HOME").expect("$HOME isn't set");

    let mut target_dir = PathBuf::from(home_dir);
    target_dir.push("src");
    target_dir.push(repository.host);
    target_dir.push(repository.path);

    fs::create_dir_all(&target_dir).expect("error creating clone directory");

    println!("Cloning repository to {:?}...", &target_dir);

    let command_output = Command::new("git")
        .arg("clone")
        .arg("--quiet")
        .arg(arg_repo)
        .arg(&target_dir)
        .output()
        .expect("error calling git");

    io::stdout()
        .write_all(&command_output.stdout)
        .expect("error writing to stdout");
    io::stderr()
        .write_all(&command_output.stderr)
        .expect("error writing to stderr");

    if command_output.status.success() {
        println!("🎉 Done!");
    }
}
