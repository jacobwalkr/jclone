mod configuration;
mod repository;

use jclone::git_clone;
use repository::Repository;
use std::{env, path::PathBuf};

use crate::configuration::Configuration;

fn main() {
    let home_dir = env::var("HOME").expect("$HOME environment variable not set");

    let mut config_path = PathBuf::from(&home_dir);
    config_path.push(".jclone.toml");

    let config = match Configuration::from_file(&config_path) {
        Ok(c) => c,
        Err(_) => Configuration::with_default_values(&home_dir)
    };

    let arg_repo = env::args().nth(1).expect("expecting argument: repository");
    let repository = Repository::try_from(&arg_repo).expect("couldn't parse repository");

    let mut target_dir = config.base_dir.clone();
    target_dir.push(repository.host);
    target_dir.push(repository.path);

    println!("Cloning repository to {:?}...", &target_dir);

    match git_clone(&arg_repo, &target_dir) {
        Ok(_) => println!("🎉 Done!"),
        Err(err) => println!("❌ Error: {}", err),
    }
}
