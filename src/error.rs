#[expect(clippy::large_enum_variant, reason = "SDK error is large currently")]
#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error("Task: {0}")]
    Task(String),
}
