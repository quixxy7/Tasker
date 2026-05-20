use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(id: u32, name: String) -> Self {
        Task {
            id,
            name,
            status: TaskStatus::Todo,
        }
    }
}

const TASKS_FILE: &str = ".tsk/tasks.json";

#[derive(Serialize, Deserialize)]
pub struct TaskStorage {
    pub tasks: Vec<Task>,
    pub next_id: u32,
}
impl TaskStorage {
    pub fn load() -> TaskStorage {
        let content = fs::read_to_string(TASKS_FILE).unwrap();
        let storage: TaskStorage = serde_json::from_str(&content).unwrap();
        storage
    }

    pub fn save(&self) {
        let content = serde_json::to_string(self).unwrap();
        fs::write(TASKS_FILE, content).unwrap();
    }
}
