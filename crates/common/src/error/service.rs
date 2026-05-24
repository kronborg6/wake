use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("invalid email address: ")]
    CreateionError,
    #[error("invalid email address: ")]
    MissingId,
    #[error("invalid email address: ")]
    InvaldeLocator,
}
