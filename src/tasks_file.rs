use crate::format::TasksFile;
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    process::exit,
};

pub fn detect_tasks_file() -> Result<PathBuf, String> {
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

pub fn parse_tasks_file(path: PathBuf) -> TasksFile {
    let content = read_to_string(&path).unwrap();

    match path.extension().and_then(|s| s.to_str()) {
        Some("toml") => {
            let parsed: TasksFile = toml::from_str(&content).unwrap_or_else(|err| {
                eprintln!("error parsing toml file: {err}");
                exit(1);
            });

            parsed
        }
        Some("yaml") | Some("yml") => {
            let parsed: TasksFile = serde_yaml::from_str(&content).unwrap_or_else(|err| {
                eprintln!("error parsing yaml file: {err}");
                exit(1);
            });

            parsed
        }
        Some("json") => {
            let parsed: TasksFile = serde_json::from_str(&content).unwrap_or_else(|err| {
                eprintln!("error parsing json file: {err}");
                exit(1);
            });

            parsed
        }
        _ => {
            eprintln!("unsupported file format.");
            exit(1);
        }
    }
}
