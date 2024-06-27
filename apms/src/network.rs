pub fn download_data(url: String) -> Result<Vec<u8>, std::io::Error> {
    let mut resp = reqwest::blocking::get(url.clone()).expect("Failed to fetch package!");
    let status = resp.status();

    if !status.is_success() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Request failed with {}", status.as_u16())));
    }

    let body = resp.bytes().expect("Failed to read bytes!");
    Ok(body.to_vec())
}

pub fn get_package_url(hostname: String, name: String) -> String {
    // Trim any trailing slashes from the hostname
    let trimmed = hostname.trim_end_matches('/');

    // Check if the trimmed hostname contains the placeholder <PKG>
    if !trimmed.contains("<PKG>") {
        // If <PKG> is not present, append the package name to the hostname
        return format!("{trimmed}/{name}");
    }

    // Replace <PKG> with the package name in the hostname
    let url = trimmed.replace("<PKG>", &name);
    
    // Return the constructed URL
    url
}