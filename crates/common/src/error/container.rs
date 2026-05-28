use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContainerError {
    #[error("failed to create container")]
    CreationError,
    #[error("missing container id")]
    MissingId,
    #[error("iinvalid container locator: {0}")]
    InvaldeLocator(String),
    #[error("failed to fetch containers: {0}")]
    FetchingError(String),
}
