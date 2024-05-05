use std::process::exit;

use crate::command::COMMANDS;

mod paths;
mod installer;
mod command;

pub fn install(args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Usage: apms install <package>");
        exit(1);
    }

    installer::install_package(args[2].clone());
}

pub fn help(_: Vec<String>) {
    // Forcefully unlock the commands mutex, so we don't get stuck indefinitely
    unsafe {
        COMMANDS.force_unlock();
    }

    for cmd in COMMANDS.lock().iter() {
        println!("apms {}: {}", cmd.name, cmd.description);
    }
}

pub fn unimplemented_command(_: Vec<String>) {
    unimplemented!()
}

fn main() {
    paths::init_paths();
    command::init_commands();

    // std::env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: apms <action> <...>");
        exit(1);
    }

    let mut cmd_found: bool = false;
    for cmd in COMMANDS.lock().iter() {
        if args[1].clone() == cmd.name {
            cmd_found = true;
            (cmd.function)(args.clone());
            break;
        }
    }

    if !cmd_found {
        eprintln!("Command \"{}\" not found!", args[1]);
        exit(1);
    }
}
