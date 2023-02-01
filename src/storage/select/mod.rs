use rmp_serde::decode as rmps;
use std::fs::{self, ReadDir};
use std::path::Path;

use super::get_base_data_path;
use crate::model::Person;

fn get_dir_files() -> Option<ReadDir> {
    let str_path = get_base_data_path();
    let dir_path = Path::new(&str_path);
    if dir_path.exists() && dir_path.is_dir() {
        let files = fs::read_dir(dir_path).unwrap();
        return Some(files);
    }
    None
}

pub fn list(quantity: usize) -> Vec<Person> {
    let files = get_dir_files();

    match files {
        None => {
            return vec![];
        }
        Some(files) => {
            let result: Vec<Person> = files
                .map(|file_path| {
                    let entry = file_path.unwrap();
                    let path = entry.path();
                    let file = fs::File::open(path).unwrap();
                    let decoded: Person = rmps::from_read(file).unwrap();
                    decoded
                })
                .take(quantity)
                .collect();

            return result;
        }
    }
}

pub fn get_by_key(key: &str) -> Option<Person> {
    let mut str_path = get_base_data_path();
    str_path.push_str(key);
    let path = Path::new(&str_path);
    if path.exists() {
        let file = fs::File::open(path).unwrap();
        let decoded: Person = rmps::from_read(file).unwrap();
        return Some(decoded);
    }
    None
}
