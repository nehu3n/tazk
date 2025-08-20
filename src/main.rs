mod execution;
mod format;
mod tasks_file;

use std::{path::PathBuf, process::exit};

use clap::Parser;

use crate::{
    execution::run_from_task,
    format::TasksFile,
    tasks_file::{ValidationError, detect_tasks_file, parse_tasks_file, validate_tasks_file},
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

    let errors = validate_tasks_file(file_parsed.clone());

    if !errors.is_empty() {
        eprintln!("validation errors found:");
        for error in errors {
            match error {
                ValidationError::DuplicatedTask(name) => {
                    eprintln!(" - duplicated task name: {name}");
                }
                ValidationError::DependencyNotFound { task, dep } => {
                    eprintln!(" - task '{task}' has a missing dependency: '{dep}'");
                }
                ValidationError::EmptyCommand(name) => {
                    eprintln!(" - task '{name}' has an empty command.");
                }
                ValidationError::SelfDependency(name) => {
                    eprintln!(" - task '{name}' has a self-dependency.");
                }
                ValidationError::CyclicDependency { cycle } => {
                    eprintln!(" - cyclic dependency detected: {}", cycle.join(" -> "));
                }
            }
        }
        exit(1);
    }

    if cli.list {
        println!("available tasks:");
        for (name, task) in file_parsed.tasks {
            println!(
                " - {name}{}",
                if let Some(desc) = task.desc { format!(": {desc}") } else { "".to_string() }
            );
        }
        exit(0);
    }

    let task_name = cli.task.clone().unwrap_or_default();
    run_from_task(&file_parsed.tasks, &task_name);
}
