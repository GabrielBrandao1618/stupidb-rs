use pest::iterators::Pairs;
use rmp_serde::decode as rmps;
use std::fs::{self, ReadDir};
use std::path::Path;

use super::get_base_data_path;
use crate::model::Person;

use crate::query::conditional_helpers::satisfies_where;
use crate::query::Rule;

fn get_dir_files() -> Option<ReadDir> {
    let str_path = get_base_data_path();
    let dir_path = Path::new(&str_path);
    if dir_path.exists() && dir_path.is_dir() {
        let files = fs::read_dir(dir_path).unwrap();
        return Some(files);
    }
    None
}

pub fn list(quantity: Option<usize>, conditions: Option<Pairs<Rule>>) -> Vec<Person> {
    let files = get_dir_files();

    match files {
        None => {
            return vec![];
        }
        Some(files) => {
            let mut result: Vec<Person> = files
                .map(|file_path| {
                    let entry = file_path.unwrap();
                    let path = entry.path();
                    let file = fs::File::open(path).unwrap();
                    let decoded: Person = rmps::from_read(file).unwrap();
                    decoded
                })
                .filter(|row| {
                    match conditions.clone() {
                        Some(val) => satisfies_where(val, row),
                        None => true
                    }
                }).collect();
                match quantity {
                    Some(val) => result = result.into_iter().take(val).collect(),
                    None => ()
                }

            return result;
        }
    }
}
