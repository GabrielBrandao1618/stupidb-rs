use clap::{Parser,Subcommand};

#[derive(Parser)]
#[clap(author="http://github.com/GabrieBrandao1618", about="A simple database to store people data")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[clap(about="Create a new record")]
    Push {
        name: String,
        age: u16,
    },
    #[clap(about="List records given a limit")]
    List {
        #[arg(long, default_value_t = 50)]
        limit: u8,
        #[arg(long)]
        min_age: Option<u16>,
        #[arg(long)]
        max_age: Option<u16>,
    },
    #[clap(about="Remove a record")]
    Remove {
        #[command(subcommand)]
        param: RemoveParam,
    },
    #[clap(about="Get a record given a key")]
    Get {
        key: String,
    },
    #[clap(about="Clear all data, erasing all records")]
    Clear
}

#[derive(Subcommand)]
pub enum RemoveParam {
    #[clap(about="Remove a record using a key")]
    Id {
        val: String,
    }
}
