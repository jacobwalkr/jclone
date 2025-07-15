mod repository;

use jclone::git_clone;
use repository::Repository;
use std::path::PathBuf;
use std::{env};

fn main() {
    let arg_repo = env::args().nth(1).expect("expecting argument: repository");
    let repository = Repository::try_from(&arg_repo).expect("couldn't parse repository");

    let home_dir = env::var("HOME").expect("$HOME isn't set");

    let mut target_dir = PathBuf::from(home_dir);
    target_dir.push("src");
    target_dir.push(repository.host);
    target_dir.push(repository.path);

    println!("Cloning repository to {:?}...", &target_dir);

    match git_clone(&arg_repo, &target_dir) {
        Ok(_) => println!("🎉 Done!"),
        Err(err) => println!("❌ Error: {}", err)
    }
}
