use std::path::Path;
use std::fs;

use rmp_serde::encode as rmps;

use crate::model::Person;
use super::get_base_data_path;

fn ensure_dir_exists() {
    let str_path = get_base_data_path();
    let dir_path = Path::new(&str_path);
    if !dir_path.exists() {
        fs::create_dir_all(dir_path).unwrap();
    }
}

pub fn create_person(name: String, age: u16) -> Person {
    ensure_dir_exists();
    let person = Person::new(name, age);

    let mut str_path = get_base_data_path();
    str_path.push_str(&person.id);
    let path = Path::new(&str_path);

    let mut buf = Vec::new();
    rmps::write(&mut buf, &person).unwrap();
    fs::write(path, buf).unwrap();
    person
}
