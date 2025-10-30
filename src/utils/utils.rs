use std::env::home_dir;
use std::path::{Path, PathBuf};

#[macro_export]
macro_rules! display_vec {
    ($name:expr) => {
        $name.iter().map(|x| x.to_string()).collect::<Vec<String>>()
    };
}

pub fn get_starting_path() -> Box<Path> {
    let home_dir = match home_dir() {
        Some(res) => res,
        None => PathBuf::new().join("/"),
    };
    let path_box = home_dir.into_boxed_path();
    path_box
}
