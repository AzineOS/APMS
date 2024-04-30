use core::panic;
use std::process::exit;

mod paths;
mod installer;

fn main() {
    paths::init_paths();

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

            installer::install_package(args[2].clone());
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