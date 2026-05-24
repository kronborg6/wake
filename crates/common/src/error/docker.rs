use thiserror::Error;

#[derive(Debug, Error)]
pub enum DockerError {
    #[error("cound not found docker instanche")]
    StartupError,
    #[error("invalid email address: ")]
    MissingId,
    #[error("invalid email address: ")]
    InvaldeLocator,
}
