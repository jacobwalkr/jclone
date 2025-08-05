use jclone::jclone;
use std::env;

fn main() {
    let arg_repo = env::args().nth(1).expect("expecting argument: repository");

    match jclone(arg_repo) {
        Ok(_) => println!("🎉 Done!"),
        Err(err) => println!("❌ Error: {}", err),
    }
}
