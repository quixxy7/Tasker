pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

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
