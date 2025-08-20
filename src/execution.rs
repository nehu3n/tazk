use crate::format::Task;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    process::{Command, exit},
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

pub fn run_from_task(tasks: &HashMap<String, Task>, start: &str) {
    let deps = collect_dependencies(tasks, start);
    let order = topological_order(tasks);

    let filtered: Vec<String> = order.into_iter().filter(|t| deps.contains(t)).collect();

    for task_name in filtered {
        if let Some(task) = tasks.get(&task_name) {
            println!("🐕 running task: {task_name}");
            run_task(&task_name, task);
        }
    }
}

fn run_task(task_name: &String, task: &Task) {
    let mut parts = task.cmd.split_whitespace();
    let cmd = parts.next().unwrap();
    let args: Vec<&str> = parts.collect();

    let status = Command::new(cmd).args(args).status().expect("command execution failed");

    if !status.success() {
        eprintln!("task '{task_name}' failed.");
        exit(1);
    }
}
