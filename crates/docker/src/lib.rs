use bollard::Docker as bDocker;

use crate::{client::connect, error::DockerError};

pub(crate) mod client;
pub mod container;
pub(crate) mod error;

struct Docker {
    pub docker: bDocker,
}

impl Docker {
    pub fn new() -> Result<Self, DockerError> {
        match connect() {
            Ok(c) => Ok(Self { docker: c }),
            Err(_) => Err(DockerError::ConnectionError),
        }
    }
}
