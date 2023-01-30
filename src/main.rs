#[macro_use] extern crate pest;

mod model;
mod cli;
mod storage;
mod query;

fn main() {
    query::parser::parse("select where name = Gabriel");    
}
