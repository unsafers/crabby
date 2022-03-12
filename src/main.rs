use clap::{Parser, Subcommand};

/// Hello, this is Crabby, your friend that plays Spotify music for you! 
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Status,
    Play,
    Pause,
    Stop,
    Connect,
    Suffle
}
fn main() {
    let args = Args::parse();

}
