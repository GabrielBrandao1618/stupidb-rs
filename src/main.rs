#[macro_use] extern crate pest;

mod model;
mod cli;
mod storage;
mod query;

use query::parser::parse;

fn main() {
    parse("select where age < 16 and name = Gabriel");    
}
