#[macro_use] extern crate pest;

mod model;
mod cli;
mod storage;
mod query;

use query::parser::parse;

fn main() {
    let result = parse("select where age < 16 and name = Gabriel");
    for row in result.rows {
        println!("{}: name: {}, age: {}", row.id, row.name, row.age);
    }
}
