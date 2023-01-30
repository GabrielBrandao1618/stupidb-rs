#[macro_use] extern crate pest;

use clap::Parser;

mod model;
mod cli;
mod storage;
mod query;

use cli::{Cli,Command,RemoveParam};
use storage::insert::create_person;
use storage::select::{list,get_by_key};
use storage::mutate::{remove_by_key,clear};

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Push { name, age } => {
            let person = create_person(name, age); 
            println!("Created person with name {}, age {}, and id {}", person.name, person.age, person.id);
        },
        Command::List {limit, min_age, max_age} => {
            let result = list(
                usize::from(limit), 
                min_age, 
                max_age
            );
            if result.len() == 0 {
                println!("No record found");
                return;
            }
            for person in result {
                println!("{}: name: {}, age: {}", person.id, person.name, person.age);
            }
        },
        Command::Remove { param } => {
            match param {
                RemoveParam::Id { val } => {
                    remove_by_key(&val);
                    println!("Removed user with id {val}");
                },
            }
        },
        Command::Get { key } => {
            let person = get_by_key(&key).expect("Record not found with the given key");
            println!("{}: name: {}, age: {}", person.id, person.name, person.age);
        },
        Command::Clear => {
           clear(); 
           println!("Removed all records");
        },
    }
}
