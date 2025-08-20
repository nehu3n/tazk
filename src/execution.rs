use crate::format::{CommandSpec, Task};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    process::{Command, exit},
    thread,
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

pub fn run_from_task(tasks: &HashMap<String, Task>, start: &str, concurrent_global: bool) {
    let deps = collect_dependencies(tasks, start);
    let order = topological_order(tasks);

    let filtered: Vec<String> = order.into_iter().filter(|t| deps.contains(t)).collect();

    for task_name in filtered {
        if let Some(task) = tasks.get(&task_name) {
            println!("🐕 running task: {task_name}");
            run_task(&task_name, task, concurrent_global);
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

    println!("   ➜ running: {cmd_str}");
    let status = command.status().expect("command execution failed");

    if !status.success() {
        eprintln!("task '{task_name}' failed on: {cmd_str}");
        exit(1);
    }
}
