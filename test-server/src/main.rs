use std::{
    env,
    fs::{self, create_dir_all},
    path::{Path, PathBuf},
};

use rocket::fs::NamedFile;

#[macro_use]
extern crate rocket;

fn get_package_directory() -> String {
    let executing_dir = env::current_exe()
        .expect("Failed to get executing directory!");
    let parent_dir = executing_dir.parent()
        .expect("Failed to get parent directory!");
    parent_dir.join("packages").to_string_lossy().to_string()
}

#[get("/<name>/download")]
async fn download_package(name: &str) -> Option<NamedFile> {
    println!("User requested: {}", name);
    let package_dir = get_package_directory();

    let path = Path::join(package_dir.as_ref(), format!("{name}/package.zip"));
    let path_str = path.to_str().expect("Failed to parse PathBuf into &str").to_string();
    NamedFile::open(path_str).await.ok()
}

#[get("/<name>/version")]
async fn get_package_version(name: &str) -> Option<String> {
    println!("User requested: {}", name);
    let package_dir = get_package_directory();

    let path = Path::join(package_dir.as_ref(), format!("{name}/version.txt"));
    let path_str = path.to_str().expect("Failed to parse PathBuf into &str");

    // Attempt to read the version from the file
    match fs::read_to_string(path_str) {
        Ok(version) => Some(version),
        Err(_) => None,
    }
}

#[launch]
fn rocket() -> _ {
    // Initialize Paths
    let packages_dir = get_package_directory();
    if !Path::exists(packages_dir.as_ref()) {
        create_dir_all(packages_dir).expect("Failed to create package directory!");
    }

    println!("{}", get_package_directory());

    rocket::build().mount("/packages", routes![download_package, get_package_version])
}