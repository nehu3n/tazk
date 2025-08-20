use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Tazk")]
#[command(version = "0.1.0")]
#[command(about = "🐕 Lightweight, agnostic, fast and easy task runner.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg()]
        name: String,
    },
    List {},
}

fn main() {
    let _cli = Cli::parse();
}
