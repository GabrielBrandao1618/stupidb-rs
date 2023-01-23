use rmp_serde::decode as rmps;
use std::fs;
use std::path::Path;

use crate::model::Person;

const BASE_PATH: &str = "./data/";

pub fn list_all() -> Vec<Person> {
    let dir_path = Path::new(BASE_PATH);
    if dir_path.is_dir() {
        let files = fs::read_dir(dir_path).unwrap();
        
        let result: Vec<Person> = files.map(|file_path| {
            let entry = file_path.unwrap();
            let path = entry.path();
            let file = fs::File::open(path).unwrap();
            let decoded: Person = rmps::from_read(file).unwrap();
            decoded
        }).collect();
        result
    } else {
        vec![]
    }
}
