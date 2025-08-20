use crate::format::{Task, TasksFile};
use std::{
    collections::{HashMap, HashSet},
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

#[derive(Debug)]
pub enum ValidationError {
    DuplicatedTask(String),
    DependencyNotFound { task: String, dep: String },
    EmptyCommand(String),
    SelfDependency(String),
    CyclicDependency { cycle: Vec<String> },
}

pub fn validate_tasks_file(file: TasksFile) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    let mut seen = HashSet::new();
    for name in file.tasks.keys() {
        if !seen.insert(name.clone()) {
            errors.push(ValidationError::DuplicatedTask(name.clone()));
        }
    }

    let task_names: HashSet<_> = file.tasks.keys().map(|name| name.as_str()).collect();
    for (name, task) in &file.tasks {
        for dep in &task.deps {
            if !task_names.contains(dep.as_str()) {
                errors.push(ValidationError::DependencyNotFound {
                    task: name.clone(),
                    dep: dep.clone(),
                });
            }
        }
    }

    for (name, task) in &file.tasks {
        if task.cmd.trim().is_empty() {
            errors.push(ValidationError::EmptyCommand(name.clone()));
        }
    }

    for (name, task) in &file.tasks {
        if task.deps.contains(name) {
            errors.push(ValidationError::SelfDependency(name.clone()));
        }
    }

    let cycles = detect_cycles(&file.tasks);
    for cycle in cycles {
        errors.push(ValidationError::CyclicDependency { cycle });
    }

    errors
}

fn detect_cycles(tasks: &HashMap<String, Task>) -> Vec<Vec<String>> {
    let mut cycles = Vec::new();
    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    fn dfs<'a>(
        task: &'a str,
        tasks: &'a HashMap<String, Task>,
        visiting: &mut HashSet<&'a str>,
        visited: &mut HashSet<&'a str>,
        stack: &mut Vec<&'a str>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        if visiting.contains(task) {
            let cycle_start = stack.iter().position(|&t| t == task).unwrap();
            let cycle: Vec<String> = stack[cycle_start..].iter().map(|t| t.to_string()).collect();
            cycles.push(cycle);
            return;
        }

        if visited.contains(task) {
            return;
        }

        visiting.insert(task);
        stack.push(task);

        if let Some(t) = tasks.get(task) {
            for dep in &t.deps {
                dfs(dep, tasks, visiting, visited, stack, cycles);
            }
        }

        stack.pop();
        visiting.remove(task);
        visited.insert(task);
    }

    for task in tasks.keys() {
        dfs(task, tasks, &mut visiting, &mut visited, &mut stack, &mut cycles);
    }

    cycles
}
