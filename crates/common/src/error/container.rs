use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContainerError {
    #[error("invalid email address: ")]
    CreateionError,
    #[error("invalid email address: ")]
    MissingId,
    #[error("invalid email address: ")]
    InvaldeLocator,
}
