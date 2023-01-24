use clap::Parser;

mod model;
mod cli;
mod store;
mod select;
mod mutate;

use cli::{Cli,Command,RemoveParam};
use store::create_person;
use select::{list,get_by_key};
use mutate::remove_by_key;

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
        }
    }
}
