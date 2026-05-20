use serde::{Deserialize, Serialize};
use std::{fmt, fs};

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "todo"),
            TaskStatus::InProgress => write!(f, "in progress"),
            TaskStatus::Done => write!(f, "done"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "id: {} | name: {} | status: {}",
            self.id, self.name, self.status
        )
    }
}

const TASKS_FILE: &str = ".tsk/tasks.json";

#[derive(Serialize, Deserialize)]
pub struct TaskStorage {
    pub tasks: Vec<Task>,
    pub next_id: u32,
}
impl TaskStorage {
    pub fn load() -> Result<TaskStorage, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(TASKS_FILE)?;
        let storage: TaskStorage = serde_json::from_str(&content)?;
        Ok(storage)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string(self)?;
        fs::write(TASKS_FILE, content)?;
        Ok(())
    }
}
