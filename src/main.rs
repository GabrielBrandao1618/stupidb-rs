use clap::Parser;

mod model;
mod cli;
mod store;

use cli::{Cli,Command};
use store::create_person;

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Push { name, age } => {
            let person = create_person(name, age); 
            println!("Created person with name {}, age {}, and id {}", person.name, person.age, person.id);
        },
        Command::List => println!("Listing all..."),
    }
}
