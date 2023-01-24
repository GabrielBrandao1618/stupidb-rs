use std::path::Path;
use std::fs::{remove_file, self};

use super::get_base_data_path;

pub fn remove_by_key(key: &str) {
    let mut path_str = get_base_data_path();
    path_str.push_str(key);
    let path = Path::new(&path_str);

    if path.exists() {
        remove_file(path).unwrap();
    }
}
pub fn clear() {
    let path_str = get_base_data_path();
    let path = Path::new(&path_str);
    if path.exists() && path.is_dir() {
        fs::remove_dir_all(path).unwrap();
    }
}
