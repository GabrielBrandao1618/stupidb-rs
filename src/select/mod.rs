use rmp_serde::decode as rmps;
use std::fs;
use std::path::Path;

use crate::model::Person;

const BASE_PATH: &str = "./data/";

pub fn list_all() {
    let dir_path = Path::new(BASE_PATH);
    if dir_path.is_dir() {
        let files = fs::read_dir(dir_path).unwrap();
        
        for file_path in files {
            let entry = file_path.unwrap();
            let path = entry.path();
            let file = fs::File::open(path).unwrap();
            let decoded: Person = rmps::from_read(file).unwrap();

            println!("name: {}, age: {}, id: {}", decoded.name, decoded.age, decoded.id); 
        } 
    }
}
