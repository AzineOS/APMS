use lazy_static::lazy_static;
use crate::paths::read_hostname;

lazy_static! {
    pub static ref DEFAULT_HOSTNAME: String = String::from("http://127.0.0.1:8080/packages/<PKG>/download");
}

fn download_data(url: String) -> Result<String, reqwest::Error> {
    reqwest::blocking::get(url)?.text()
}

fn get_url(hostname: String, name: String) -> String {
    let trimmed = hostname.trim_end_matches('/');
    if !trimmed.contains("<PKG>") {
        dbg!(trimmed);
        return format!("{trimmed}/{name}");
    }

    // Replace <PKG> with the name
    let url = trimmed.replace("<PKG>", &*name);
    url
}

pub fn install_package(name: String) {
    let hostname = read_hostname()
        .expect("Failed to read hostname!");

    let request_url = get_url(hostname, name);
    println!("Reading: {}", request_url.clone());

    let response = download_data(request_url)
        .expect("Failed to download package data!");

    println!("Received data: {}", response);
}