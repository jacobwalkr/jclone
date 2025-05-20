use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

fn main() {
    let arg_repo = env::args().nth(1).expect("expecting argument: repository");

    let repository = parse_repo_string(&arg_repo).expect("couldn't parse repository");

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

struct Repository {
    host: String,
    path: String,
}

fn parse_repo_string(repo_str: &str) -> Result<Repository, &str> {
    let part_after_proto = match repo_str.split_once("://") {
        None => repo_str,
        Some((_, p)) => p,
    };

    let (prefix, suffix) = match part_after_proto.split_once(':') {
        Some(("", _)) | Some((_, "")) | None => return Err("unexpected format"),
        Some((prefix, _)) if prefix.contains('/') => return Err("looks like local path"),
        Some(parts) => parts,
    };

    let host_part = match prefix.split_once('@') {
        None => prefix,
        Some((_, host)) => host,
    };

    let path_part_trimmed = suffix.trim_start_matches('/');

    let path_part = match path_part_trimmed.strip_suffix(".git") {
        None => path_part_trimmed,
        Some(p) => p,
    };

    Ok(Repository {
        host: host_part.to_owned(),
        path: path_part.to_owned(),
    })
}
