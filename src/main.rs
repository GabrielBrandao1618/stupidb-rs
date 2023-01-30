#[macro_use] extern crate pest;

mod model;
mod cli;
mod storage;
mod query;

use query::parser::parse;

fn main() {
    parse("select where name = Gabriel and age > 18");    
    parse("delete where name = Gabriel");    
    parse("insert name = Gabriel, age = 18");    
}
