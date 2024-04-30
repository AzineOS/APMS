use lazy_static::lazy_static;
use directories::ProjectDirs;

lazy_static! {
    pub static ref PROJECT_DIR: ProjectDirs = ProjectDirs::from("com", "azine", "apms").expect("Failed to initialize PROJECT_DIR");
}

pub fn init_paths() {
    let path = PROJECT_DIR.config_dir().to_str().expect("Failed to parse PROJECT_DIR");
    println!("{}", PROJECT_DIR.data_local_dir().to_str().unwrap());
    std::fs::create_dir_all(path).expect("Failed to create path");
}