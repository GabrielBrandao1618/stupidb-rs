use clap::Parser;

mod model;
mod cli;
mod store;
mod select;
mod mutate;

use cli::{Cli,Command,RemoveParam};
use store::create_person;
use select::{list};
use mutate::remove_by_key;

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Push { name, age } => {
            let person = create_person(name, age); 
            println!("Created person with name {}, age {}, and id {}", person.name, person.age, person.id);
        },
        Command::List {quantity} => {
            let result = list(usize::from(quantity));
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
        }
    }
}
