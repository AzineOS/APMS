use std::{fs::File, io::{self, Read, Write}, path::Path};

use directories::ProjectDirs;
use lazy_static::lazy_static;

use crate::installer::DEFAULT_HOSTNAME;

lazy_static! {
    pub static ref PROJECT_DIR: ProjectDirs = ProjectDirs::from("com", "azine", "apms").expect("Failed to initialize PROJECT_DIR");
}

pub fn get_hostname_path() -> std::path::PathBuf {
    Path::join(PROJECT_DIR.config_dir(), "hosts.txt")
}

pub fn get_packages_path() -> std::path::PathBuf {
    Path::join(PROJECT_DIR.data_dir(), "intermediate/")
}

pub fn get_pkg_path() -> std::path::PathBuf {
    // std::path::PathBuf::from("/usr/azine/bin")
    Path::join(PROJECT_DIR.data_dir(), "bin")
}

pub fn init_paths() {
    // Config Directory
    let config_path = PROJECT_DIR.config_dir();
    std::fs::create_dir_all(config_path).expect("Failed to create config path!");

    let package_data_path = get_packages_path();
    std::fs::create_dir_all(package_data_path).expect("Failed to create package data path!");

    let package_bin_path = get_pkg_path();
    std::fs::create_dir_all(package_bin_path).expect("Failed to create package bin path!");

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