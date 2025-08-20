mod format;
mod tasks_file;

use std::{path::PathBuf, process::exit};

use clap::Parser;

use crate::{
    format::TasksFile,
    tasks_file::{detect_tasks_file, parse_tasks_file},
};

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
                exit(1);
            }

            path
        }
        None => detect_tasks_file().unwrap_or_else(|err| {
            eprintln!("error: {err}");
            exit(1);
        }),
    };

    println!("file path: {}", tasks_file.display());

    let file_parsed: TasksFile = parse_tasks_file(tasks_file);
    println!("parsed tasks file: {file_parsed:?}");
}
