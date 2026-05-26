#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("container not found")]
    ContainerNotFound,
    #[error("conection failed with {0}")]
    ConcationFailed(String),
    #[error("intanaly run time error: {0}")]
    Internal(String),
    #[error("failed to map to container")]
    MapError,
}
