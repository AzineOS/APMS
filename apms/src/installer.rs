use std::fmt::format;
use std::fs::File;
use std::io::{Read, Write};
use lazy_static::lazy_static;
use crate::paths::{get_packages_path, PROJECT_DIR, read_hostname};

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

pub fn install_package(name: String) {
    let hostname = read_hostname()
        .expect("Failed to read hostname!");

    let request_url = get_url(hostname, name.clone());
    println!("Reading: {}", request_url.clone());

    let response = download_data(request_url)
        .expect("Failed to download package data!");

    println!("Package data received! Writing to file...");

    let data_dir = get_packages_path().join(format!("{name}"));

    // Create package directory
    std::fs::create_dir_all(data_dir.clone())
        .expect("Failed to create directory for package!");

    // Download package ZIP
    let mut package_zip = File::create(data_dir.join("package.zip"))
        .expect("Failed to create package.zip!");
    package_zip.write(response.leak())
        .expect("Failed to write to package.zip!");

    // TODO: Extract ZIP
}