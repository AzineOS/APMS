use std::{fs::File, io::{self, ErrorKind, Read, Write}, path::Path};

use lazy_static::lazy_static;
use directories::ProjectDirs;

use crate::installer::DEFAULT_HOSTNAME;

lazy_static! {
    pub static ref PROJECT_DIR: ProjectDirs = ProjectDirs::from("com", "azine", "apms").expect("Failed to initialize PROJECT_DIR");
}

pub fn get_hostname_path() -> std::path::PathBuf {
    Path::join(PROJECT_DIR.config_dir(), "hosts.txt")
}

pub fn init_paths() {
    // Config Directory
    let path = PROJECT_DIR.config_dir();
    std::fs::create_dir_all(path).expect("Failed to create path");

    // Hostnames File
    let hostname_path = get_hostname_path();
    if hostname_path.exists() {
        return
    }

    let mut hostname_file = File::create(hostname_path).expect("Failed to create hosts.txt");
    hostname_file.write(DEFAULT_HOSTNAME.as_bytes()).expect("Failed to write default hostname to hosts.txt");
}

pub fn read_hostname() -> Result<String, std::io::Error> {
    let path = get_hostname_path();
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Hosts.txt not found"));
    }

    let mut hostname_file = File::open(path)?;
    let mut read = String::new();
    hostname_file.read_to_string(&mut read)?;

    if let Some(index) = read.find('\n') {
        read.truncate(index);
    }
    Ok(read)
}