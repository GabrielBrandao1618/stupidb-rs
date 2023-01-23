use std::path::Path;
use std::fs::remove_file;

const BASE_PATH: &str = "./data/";

pub fn remove_by_key(key: &str) {
    let mut path_str = BASE_PATH.to_owned();
    path_str.push_str(key);
    let path = Path::new(&path_str);

    if path.exists() {
        remove_file(path).unwrap();
    }
}
