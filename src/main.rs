#[macro_use]
extern crate pest;

mod cli;
mod model;
mod query;
mod storage;

use query::parser::parse;

fn main() {
    let result = parse("select where age > 13 and age < 15 limit 2");
    for row in result.rows {
        println!("{}: name: {}, age: {}", row.id, row.name, row.age);
    }
}
