#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("container not found")]
    ContainerNotFound,
    #[error("connection failed: {0}")]
    ConnectionFailed(String),
    #[error("internal runtime error")]
    Internal,
    #[error("failed to map to container")]
    MapError,
}
