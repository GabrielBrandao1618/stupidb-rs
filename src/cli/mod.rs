use clap::Parser;

#[derive(Parser)]
#[clap(
    author = "http://github.com/GabrieBrandao1618",
    about = "A simple database to store people data"
)]
pub struct Cli {
    pub command: String,
}
