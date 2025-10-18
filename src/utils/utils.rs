use std::env::home_dir;
use std::path::{Path, PathBuf};

pub fn get_starting_path() -> Box<Path> {
    let home_dir = match home_dir() {
        Some(res) => res,
        None => PathBuf::new().join("/"),
    };
    let path_box = home_dir.into_boxed_path();
    path_box
}
