use clap::Parser;

mod model;
mod cli;
mod store;

use cli::{Cli,Command};

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Push { name, age } => println!("Storing person with name {name} and age {age}"),
        Command::List => println!("Listing all..."),
    }
}
