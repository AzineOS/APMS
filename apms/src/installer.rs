use crate::{network::*, paths::{get_pkg_path, get_packages_path, read_hostname}};
use std::{fs::{self, File}, io::{Cursor, Read, Write}, process::exit};
use std::path::{Path, PathBuf};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEFAULT_HOSTNAME: String = String::from("http://127.0.0.1:8080/packages/<PKG>/download");
}

fn extract_zip(file_path: &Path, extract_path: &Path) -> Result<(), zip_extract::ZipExtractError> {
    // Read the contents of the ZIP file
    let mut zip_file = File::open(file_path)?;
    let mut zip_data = Vec::new();
    zip_file.read_to_end(&mut zip_data)?;

    // Extract the contents of the ZIP file to the provided path
    zip_extract::extract(Cursor::new(zip_data), extract_path, true)?;

    Ok(())
}

pub fn install_package(name: String) {
    let hostname = match read_hostname() {
        Ok(hostname) => hostname,
        Err(err) => {
            eprintln!("Failed to read hostname: {}", err);
            return;
        }
    };

    let request_url = get_package_url(hostname, name.clone());
    println!("Reading: {}", request_url);

    let response = match download_data(request_url) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Failed to download package data: {}", err);
            return;
        }
    };

    println!("Package data received! Writing to file...");

    let data_dir = get_packages_path().join(&name);
    println!("Downloading to {}", data_dir.to_str().unwrap());

    if let Err(err) = fs::create_dir_all(&data_dir) {
        eprintln!("Failed to create directory for package: {}", err);
        return;
    }

    let package_path = data_dir.join("package.zip");
    let mut package_zip = match File::create(&package_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to create package.zip: {}", err);
            return;
        }
    };

    if let Err(err) = package_zip.write_all(&response) {
        eprintln!("Failed to write to package.zip: {}", err);
        return;
    }

    let extract_path = get_pkg_path().join(format!("{}", name));
    if let Err(err) = extract_zip(&package_path, &extract_path) {
        eprintln!("Extraction failed: {}", err);
    } else {
        println!("Extraction successful!");
    }

    // Remove package zip
    std::fs::remove_dir_all(data_dir).expect("Failed to remove temporary package directory");
}

pub fn remove_package(name: String) {
    // Check if directory with name exists
    let removal_dir = get_pkg_path().join(format!("{}", name));

    if !removal_dir.exists() {
        eprintln!("Package doesn't exist on system");
        exit(1);
    }

    // Make it not exist
    std::fs::remove_dir_all(removal_dir).expect("Failed to delete package directory");
}

pub fn find_package(name: String) {
    // Check if directory with name exists
    let find_dir = get_pkg_path().join(format!("{}", name));

    if !find_dir.exists() {
        eprintln!("Package doesn't exist on system");
        exit(1);
    }

    println!("Package {} found at: {}", name, find_dir.to_str().unwrap());
}