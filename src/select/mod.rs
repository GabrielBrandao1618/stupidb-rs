use rmp_serde::decode as rmps;
use std::fs::{self, ReadDir};
use std::path::Path;

use crate::model::Person;

const BASE_PATH: &str = "./data/";

fn get_dir_files() -> Option<ReadDir> {
    let dir_path = Path::new(BASE_PATH);
    if dir_path.exists() && dir_path.is_dir() {
        let files = fs::read_dir(dir_path).unwrap();
        return Some(files)
    }
    None
}

pub fn list(quantity: usize) -> Vec<Person> {
    let files = get_dir_files().unwrap();

    let result: Vec<Person> = files.map(|file_path| {
        let entry = file_path.unwrap();
        let path = entry.path();
        let file = fs::File::open(path).unwrap();
        let decoded: Person = rmps::from_read(file).unwrap();
        decoded
    }).take(quantity).collect();
    result
}

pub fn get_by_key(key: &str) -> Option<Person>{
    let mut str_path = BASE_PATH.to_owned();
    str_path.push_str(key);
    let path = Path::new(&str_path);
    if path.exists() {
        let file = fs::File::open(path).unwrap();
        let decoded: Person = rmps::from_read(file).unwrap();
        return Some(decoded);
    }
    None
}
