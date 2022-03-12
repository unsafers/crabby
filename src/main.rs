use clap::{Parser, Subcommand};

/// Hello, this is Crabby, your friend that plays Spotify music for you! 
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Connect,
    Disconnect,
    GetPlayback,
    List {
        resource: String,
    },
    Next,
    Pause,
    Play {
        uri: Option<String>,
    },
    Previous,
    Search,
    Status,
    Stop,
    Suffle,
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
}
