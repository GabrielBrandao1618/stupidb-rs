#[macro_use]
extern crate pest;

mod cli;
mod model;
mod query;
mod storage;

use clap::Parser;
use query::parser::parse;
use cli::Cli;

fn main() {
    let args = Cli::parse();
    let result = parse(&args.command);

    for row in result.rows {
        println!("{}: name: {}, age: {}", row.id, row.name, row.age);
    }
}
