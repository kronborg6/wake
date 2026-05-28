use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("creation failed")]
    CreateionError,
    #[error("missing id")]
    MissingId,
    #[error("invalid email address: ")]
    InvaldeLocator,
    #[error("invalid locator")]
    InvalidLocator,
}
