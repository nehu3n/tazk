use crate::{
    format::{CommandSpec, Task},
    logger::Logger,
    watch::watch_task,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    process::{Command, exit},
    sync::Arc,
    thread,
    time::Duration,
};

pub fn topological_order(tasks: &HashMap<String, Task>) -> Vec<String> {
    // dep -> task
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut indegree: HashMap<String, usize> = HashMap::new();

    for (name, task) in tasks {
        indegree.entry(name.clone()).or_insert(0);

        for dep in &task.deps {
            graph.entry(dep.clone()).or_default().push(name.clone());
            *indegree.entry(name.clone()).or_insert(0) += 1;
        }
    }

    let mut queue: VecDeque<String> =
        indegree.iter().filter(|(_, deg)| **deg == 0).map(|(n, _)| n.clone()).collect();

    let mut result = Vec::new();

    while let Some(node) = queue.pop_front() {
        result.push(node.clone());

        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                let deg = indegree.get_mut(neighbor).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    result
}

pub fn collect_dependencies(tasks: &HashMap<String, Task>, start: &str) -> HashSet<String> {
    let mut visited = HashSet::new();

    fn dfs(task: &str, tasks: &HashMap<String, Task>, visited: &mut HashSet<String>) {
        if !visited.insert(task.to_string()) {
            return;
        }
        if let Some(t) = tasks.get(task) {
            for dep in &t.deps {
                dfs(dep, tasks, visited);
            }
        }
    }

    dfs(start, tasks, &mut visited);
    visited
}

fn collect_dependents(tasks: &HashMap<String, Task>, changed: &str) -> HashSet<String> {
    let mut dependents = HashSet::new();

    for (name, task) in tasks {
        if task.deps.contains(&changed.to_string()) {
            dependents.insert(name.clone());
            dependents.extend(collect_dependents(tasks, name));
        }
    }

    dependents
}

pub fn run_from_task(tasks: &HashMap<String, Task>, start: &str, concurrent_global: bool) {
    let deps = collect_dependencies(tasks, start);
    let order = topological_order(tasks);
    let filtered: Vec<String> = order.into_iter().filter(|t| deps.contains(t)).collect();

    let mut has_watchers = false;
    let tasks_arc = Arc::new(tasks.clone());

    for task_name in filtered {
        if let Some(task) = tasks.get(&task_name) {
            Logger::task_start(&task_name);

            if !task.watch.is_empty() {
                has_watchers = true;

                let task_name_clone = task_name.clone();
                let task_clone = task.clone();
                let tasks_clone = tasks_arc.clone();
                let concurrent_global_clone = concurrent_global;

                let watch = task_clone.watch.clone();
                let watch_debounce = task_clone.watch_debounce;
                let watch_propagate = task_clone.watch_propagate;
                let task_for_run = task_clone.clone();

                thread::spawn(move || {
                    watch_task(&watch, watch_debounce, move || {
                        Logger::task_start(&format!("♻️  {}", &task_name_clone));
                        run_task(&task_name_clone, &task_for_run, concurrent_global_clone);

                        if watch_propagate {
                            let dependents = collect_dependents(&tasks_clone, &task_name_clone);
                            for dep_name in dependents {
                                if let Some(dep_task) = tasks_clone.get(&dep_name) {
                                    Logger::dependency_propagated(&dep_name);
                                    run_task(&dep_name, dep_task, concurrent_global_clone);
                                }
                            }
                        }
                    });
                });
            }

            run_task(&task_name, task, concurrent_global);
        }
    }

    if has_watchers {
        Logger::waiting();
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    }
}

fn run_task(task_name: &String, task: &Task, concurrent_global: bool) {
    let commands = match &task.cmd {
        CommandSpec::Single(s) => vec![s.clone()],
        CommandSpec::Multiple(list) => list.clone(),
    };

    let concurrent = task.concurrent.unwrap_or(concurrent_global);

    if concurrent {
        let handles: Vec<_> = commands
            .into_iter()
            .map(|cmd_str| {
                let task_name = task_name.to_owned();
                let env = task.env.clone();
                thread::spawn(move || {
                    execute_command(&task_name, &cmd_str, &env);
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    } else {
        for cmd_str in commands {
            execute_command(task_name, &cmd_str, &task.env);
        }
    }
}

fn execute_command(task_name: &str, cmd_str: &str, env: &HashMap<String, String>) {
    #[cfg(unix)]
    let mut command = Command::new("sh");

    #[cfg(windows)]
    let mut command = Command::new("cmd");

    #[cfg(unix)]
    {
        command.arg("-c").arg(cmd_str);
    }

    #[cfg(windows)]
    {
        command.arg("/C").arg(cmd_str);
    }

    for (k, v) in env {
        command.env(k, v);
    }

    Logger::command(cmd_str);
    let status = command.status().expect("command execution failed");

    if !status.success() {
        Logger::error(&format!("task '{task_name}' failed on: {cmd_str}"));
        exit(1);
    }
}
