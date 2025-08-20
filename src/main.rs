mod format;

use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use clap::Parser;

use crate::format::TasksFile;

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

    println!("file path: {}", tasks_file.display());

    let file_parsed: TasksFile = parse_tasks_file(tasks_file);
    println!("parsed tasks file: {file_parsed:?}");
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

fn parse_tasks_file(path: PathBuf) -> TasksFile {
    let content = read_to_string(&path).unwrap();

    match path.extension().and_then(|s| s.to_str()) {
        Some("toml") => {
            let parsed: TasksFile = toml::from_str(&content).unwrap_or_else(|err| {
                eprintln!("error parsing toml file: {err}");
                std::process::exit(1);
            });

            parsed
        }
        Some("yaml") | Some("yml") => {
            let parsed: TasksFile = serde_yaml::from_str(&content).unwrap_or_else(|err| {
                eprintln!("error parsing yaml file: {err}");
                std::process::exit(1);
            });

            parsed
        }
        Some("json") => {
            let parsed: TasksFile = serde_json::from_str(&content).unwrap_or_else(|err| {
                eprintln!("error parsing json file: {err}");
                std::process::exit(1);
            });

            parsed
        }
        _ => {
            eprintln!("unsupported file format.");
            std::process::exit(1);
        }
    }
}
