use jclone::jclone;
use std::env;

fn main() {
    let arg_repo = env::args().nth(1).expect("expecting argument: repository");
    jclone(arg_repo);
}
