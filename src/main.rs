mod format;

use std::path::{Path, PathBuf};

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
    let cli = Cli::parse();

    let tasks_file = match cli.file {
        Some(file) => {
            let path = PathBuf::from(file);
            if !path.exists() {
                eprintln!("error: the specified file does not exist: {}", path.display());
                std::process::exit(1);
            }

            path
        }
        None => detect_tasks_file().unwrap_or_else(|err| {
            eprintln!("error: {err}");
            std::process::exit(1);
        }),
    };

    println!("{}", tasks_file.display());
}

fn detect_tasks_file() -> Result<PathBuf, String> {
    let path = Path::new(".");
    let expected_files = ["tasks.toml", "tasks.yaml", "tasks.yml", "tasks.json"];

    for expected in expected_files {
        let file = path.join(expected);
        if file.exists() {
            return Ok(file);
        }
    }

    Err("no compatible file was found (tasks.toml, tasks.yaml, tasks.yml, tasks.json).".to_string())
}
