use bollard::Docker;

use crate::error::DockerError;

pub fn connect() -> Result<Docker, DockerError> {
    match Docker::connect_with_unix_defaults() {
        Ok(docker) => Ok(docker),
        Err(_) => Err(DockerError::ConnectionError),
    }
}
