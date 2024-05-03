use core::panic;
use std::process::exit;

mod paths;
mod installer;

fn main() {
    paths::init_paths();

    // std::env::set_var("RUST_BACKTRACE", "1");

    // TODO: Execute action with relevant args
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: apms <action> <...>");
        exit(1);
    }

    match args[1].as_str() {
        "install" => {
            if args.len() < 3 {
                println!("Usage: apms install <package>");
                exit(1);
            }
        },
        "help" => {
            if args.len() < 3 {
                println!("apms install <package>: Installs the package you specify.")
                println!("apms update: Updates packages on your system.")
                println!("apms remove: Removes a package from your system.")
                println!("apms find: Searches your system and finds a package!")
                println!("apms help: Opens this help menu! Duh!")
                exit(1);
            }
        },
        "update" => {
            todo!()
        },
        "remove" => {
            todo!()
        },
        "find" => {
            todo!()
        },
        _ => panic!("Invalid Argument"),
    }
}
