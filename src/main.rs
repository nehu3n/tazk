mod format;

use clap::Parser;

#[derive(Parser)]
#[command(name = "Tazk")]
#[command(version = "0.1.0")]
#[command(about = "🐕 Lightweight, agnostic, fast and easy task runner.", long_about = None)]
struct Cli {
    task: Option<String>,

    #[arg(long, short)]
    file: Option<String>,

    #[arg(long, short)]
    list: bool,
}

fn main() {
    let _cli = Cli::parse();
}
