use std::fmt::{Debug, Formatter};
use lazy_static::lazy_static;
use spin::mutex::Mutex;

lazy_static! {
    pub static ref COMMANDS: Mutex<Vec<Command>> = Mutex::new(Vec::new());
}

pub struct Command {
    pub name: String,
    pub description: String,
    pub function: fn(Vec<String>)
}

impl Command {
    pub const fn new(name: String, description: String, function: fn(Vec<String>)) -> Self {
        Command {
            name,
            description,
            function
        }
    }

    pub fn from_str(name: &str, description: &str, function: fn(Vec<String>)) -> Self {
        Self::new(name.to_string(), description.to_string(), function)
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("function", &"fn(Vec<String>)")
            .finish()
    }
}

pub fn add_command(cmd: Command) {
    COMMANDS.lock().push(cmd);
}

pub fn init_commands() {
    add_command(Command::from_str("help", "Shows this page", crate::help));
    add_command(Command::from_str("install", "Installs the specified package", crate::install));
    add_command(Command::from_str("update", "Updates a specified package", crate::unimplemented_command));
    add_command(Command::from_str("remove", "Uninstalls a package from your system", crate::remove));
    add_command(Command::from_str("find", "Checks your system for a package and returns the path if found", crate::find));
}