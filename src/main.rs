use std::env;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let clone_args = env::args().skip(1);
    let path = env::current_dir().unwrap();

    let command_output = Command::new("git")
        .current_dir(path)
        .arg("clone")
        .args(clone_args)
        .output()
        .expect("error calling git");

    io::stdout()
        .write_all(&command_output.stdout)
        .expect("error writing to stdout");
    io::stderr()
        .write_all(&command_output.stderr)
        .expect("error writing to stderr");
}
