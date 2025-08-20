use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CommandSpec {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    pub cmd: CommandSpec,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub deps: Vec<String>,

    #[serde(default)]
    pub watch: Vec<String>,

    #[serde(default)]
    pub cache: bool,

    #[serde(default = "default_debounce")]
    pub watch_debounce: u64,

    #[serde(default)]
    pub watch_propagate: bool,

    #[serde(default)]
    pub env: HashMap<String, String>,
}

fn default_debounce() -> u64 {
    500
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TasksFile {
    #[serde(default)]
    pub tasks: HashMap<String, Task>,
}
