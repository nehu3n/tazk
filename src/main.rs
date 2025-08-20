mod execution;
mod format;
mod logger;
mod tasks_file;
mod watch;

use crate::{
    execution::run_from_task,
    format::TasksFile,
    logger::Logger,
    tasks_file::{ValidationError, detect_tasks_file, parse_tasks_file, validate_tasks_file},
};
use clap::Parser;
use std::{path::PathBuf, process::exit};

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
    Logger::banner();

    let cli = Cli::parse();

    let tasks_file = match cli.file {
        Some(file) => {
            let path = PathBuf::from(file);
            if !path.exists() {
                Logger::error(&format!("the specified file does not exist: {}", path.display()));
                exit(1);
            }
            path
        }
        None => detect_tasks_file().unwrap_or_else(|err| {
            Logger::error(&err);
            exit(1);
        }),
    };

    Logger::file_path(&tasks_file.display().to_string());

    let file_parsed: TasksFile = parse_tasks_file(tasks_file);
    let concurrent_global = file_parsed.config.concurrent;

    let errors = validate_tasks_file(file_parsed.clone());
    if !errors.is_empty() {
        Logger::error("validation errors found:");
        for error in errors {
            match error {
                ValidationError::DuplicatedTask(name) => {
                    Logger::validation_error(&format!("duplicated task name: {name}"));
                }
                ValidationError::DependencyNotFound { task, dep } => {
                    Logger::validation_error(&format!(
                        "task '{task}' has a missing dependency: '{dep}'"
                    ));
                }
                ValidationError::EmptyCommand(name) => {
                    Logger::validation_error(&format!("task '{name}' has an empty command"));
                }
                ValidationError::SelfDependency(name) => {
                    Logger::validation_error(&format!("task '{name}' has a self-dependency"));
                }
                ValidationError::CyclicDependency { cycle } => {
                    Logger::validation_error(&format!(
                        "cyclic dependency detected: {}",
                        cycle.join(" → ")
                    ));
                }
            }
        }
        exit(1);
    }

    Logger::validation_ok();

    if cli.list {
        Logger::separator();
        Logger::available_tasks();
        for (name, task) in file_parsed.tasks {
            Logger::task_item(&name, task.desc.as_ref());
        }
        Logger::separator();
        exit(0);
    }

    let task_name = cli.task.clone().or(file_parsed.config.default.clone()).unwrap_or_default();

    Logger::separator();
    run_from_task(&file_parsed.tasks, &task_name, concurrent_global);
}
