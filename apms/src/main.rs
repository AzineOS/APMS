use std::process::exit;

mod paths;

// This function quits the application if the user is not root
fn quit_if_not_root() {
    if !nix::unistd::getuid().is_root() {
        println!("You cannot execute this program unless you are root");
        exit(1);
    }
}

fn main() {
    paths::init_paths();

    // TODO: Execute action with relevant args
    let args: Vec<String> = std::env::args().collect();
    dbg!(args);
}