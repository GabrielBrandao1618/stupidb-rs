use clap::{Parser,Subcommand};

#[derive(Subcommand)]
pub enum Command {
    Push {
        name: String,
        age: u16,
    },
    List,
}

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
