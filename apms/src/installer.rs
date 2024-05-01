use lazy_static::lazy_static;
use crate::paths::read_hostname;

lazy_static! {
    pub static ref DEFAULT_HOSTNAME: String = String::from("127.0.0.1:8080");
}

fn download_data(url: String) -> Result<String, reqwest::Error> {
    reqwest::blocking::get(url)?.text()
}

pub fn install_package(name: String) {
    let hostname = read_hostname()
        .expect("Failed to read hostname!");

    let request_url = format!("http://{hostname}/package/{name}");
    println!("Reading: {}", request_url.clone());

    let response = download_data(request_url)
        .expect("Failed to download package data!");

    println!("Received data: {}", response);
}