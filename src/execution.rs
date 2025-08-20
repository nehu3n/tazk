use crate::format::Task;
use std::collections::{HashMap, VecDeque};

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
