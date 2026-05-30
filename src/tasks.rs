use chrono;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{fmt, fs};

// TaskStatus
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl std::str::FromStr for TaskStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "todo" => Ok(TaskStatus::Todo),
            "in-progress" => Ok(TaskStatus::InProgress),
            "done" => Ok(TaskStatus::Done),
            other => Err(format!("Unknown status {}", other)),
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "{}", "todo".red()),
            TaskStatus::InProgress => write!(f, "{}", "in progress".yellow()),
            TaskStatus::Done => write!(f, "{}", "done".green()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub status: TaskStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Task
impl Task {
    pub fn new(id: u32, name: String) -> Self {
        Task {
            id,
            name,
            status: TaskStatus::Todo,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let task_info = format!(
            "{:<3} | {:<20} | {:<15}",
            self.id.to_string().red().bold(),
            self.name,
            self.status
        );
        let created = self.created_at.format("%Y-%m-%d %H:%M").to_string();
        let updated = self.updated_at.format("%Y-%m-%d %H:%M").to_string();
        let time_info = if created == updated {
            format!("{}: {}", "updated".dimmed(), updated)
        } else {
            format!(
                "{}: {} | {}: {}",
                "created_at".dimmed(),
                created,
                "updated".dimmed(),
                updated
            )
        };
        write!(f, "{} -- {}", task_info, time_info)
    }
}

// Task Storage
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
