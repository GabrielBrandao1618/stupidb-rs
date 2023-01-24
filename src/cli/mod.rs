use clap::{Parser,Subcommand};

#[derive(Subcommand)]
pub enum RemoveParam {
    Id {
        val: String,
    }
}

#[derive(Subcommand)]
pub enum Command {
    Push {
        name: String,
        age: u16,
    },
    List {
        #[arg(long, default_value_t = 50)]
        amount: u8,
        #[arg(long)]
        min_age: Option<u16>,
        #[arg(long)]
        max_age: Option<u16>,
    },
    Remove {
        #[command(subcommand)]
        param: RemoveParam,
    },
    Get {
        key: String,
    }
}

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
