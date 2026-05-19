use bollard::Docker;

use crate::error::DockerError;

/// Attempts to create a Docker client using the system's Unix socket defaults.
///
/// On success returns the connected `Docker` instance. On failure returns `DockerError::ConncationError`.
///
/// # Examples
///
/// ```
/// let docker = connect().unwrap();
/// // use `docker`...
/// ```
pub fn connect() -> Result<Docker, DockerError> {
    match Docker::connect_with_unix_defaults() {
        Ok(docker) => Ok(docker),
        Err(_) => Err(DockerError::ConncationError),
    }
}
