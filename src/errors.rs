//! Structure to handle errors

#[derive(Debug, thiserror::Error)]
pub enum TskError {
    #[error("Not initialized. Run \"tsk init\"")]
    NotInitialized,
    #[error("Task not found. Incorrect ID")]
    TaskNotFound,
    #[error("Already initialized")]
    AlreadyInitialized,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    ParseError(String),
}
