use clap::Parser;

mod model;
mod cli;
mod store;
mod select;

use cli::{Cli,Command};
use store::create_person;
use select::list_all;

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Push { name, age } => {
            let person = create_person(name, age); 
            println!("Created person with name {}, age {}, and id {}", person.name, person.age, person.id);
        },
        Command::List => {
            list_all();
        }
    }
}
