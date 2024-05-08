use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};

use lazy_static::lazy_static;

use crate::paths::{self, get_azine_bin_path, get_packages_path, read_hostname};

lazy_static! {
    pub static ref DEFAULT_HOSTNAME: String = String::from("http://127.0.0.1:8080/packages/<PKG>/download");
}

fn download_data(url: String) -> Result<Vec<u8>, reqwest::Error> {
    let mut resp = reqwest::blocking::get(url).expect("Request failed!");
    let body = resp.bytes()?;
    Ok(body.to_vec())
}

fn get_url(hostname: String, name: String) -> String {
    let trimmed = hostname.trim_end_matches('/');
    if !trimmed.contains("<PKG>") {
        return format!("{trimmed}/{name}");
    }

    // Replace <PKG> with the name
    let url = trimmed.replace("<PKG>", &*name);
    url
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
    let hostname = read_hostname()
        .expect("Failed to read hostname!");

    let request_url = get_url(hostname, name.clone());
    println!("Reading: {}", request_url.clone());

    let response = download_data(request_url)
        .expect("Failed to download package data!");

    println!("Package data received! Writing to file...");

    let data_dir = get_packages_path().join(format!("{name}"));
    println!("Downloading to {}", data_dir.clone().to_str().unwrap());

    // Create package directory
    std::fs::create_dir_all(data_dir.clone())
        .expect("Failed to create directory for package!");

    // Download package ZIP
    let package_path = data_dir.join("package.zip");
    let mut package_zip = File::create(package_path.clone())
        .expect("Failed to create package.zip!");
    package_zip.write(response.leak())
        .expect("Failed to write to package.zip!");


    let extract_path = get_azine_bin_path().join(format!("{}-archive", name));
    match extract_zip(&package_path, &extract_path) {
        Ok(()) => println!("Extraction successful!"),
        Err(err) => eprintln!("Extraction failed: {}", err),
    }
}