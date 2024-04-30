use lazy_static::lazy_static;

use crate::paths::read_hostname;

lazy_static! {
    pub static ref DEFAULT_HOSTNAME: String = String::from("127.0.0.1:8080");
}

pub fn install_package(name: String) {
    dbg!(read_hostname());
}