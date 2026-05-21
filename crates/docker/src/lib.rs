use async_trait::async_trait;
use bollard::Docker as bDocker;
use common::port::container::ContainerRuntime;
use common::{domain::container::Container, error::container::ContainerError};
mod temp;

use crate::{client::connect, container::get_all_containers, error::DockerError};

pub(crate) mod client;
pub(crate) mod container;
pub(crate) mod error;

#[derive(Debug)]
pub struct DockerRuntime {
    pub docker: bDocker,
}

impl DockerRuntime {
    pub fn new() -> Result<Self, DockerError> {
        match connect() {
            Ok(c) => Ok(Self { docker: c }),
            Err(_) => Err(DockerError::ConnectionError),
        }
    }
}

#[async_trait]
impl ContainerRuntime for DockerRuntime {
    async fn containers(&self) -> Result<Vec<Container>, ContainerError> {
        get_all_containers(&self.docker, None).await
    }
    async fn get(&self, locator: &str) -> Result<Option<Container>, ()> {
        todo!()
    }
    async fn update_restart_policy(&self, locator: &str) -> Result<Option<Container>, ()> {
        todo!()
    }
    async fn update_state(&self, locator: &str) -> Result<Option<Container>, ()> {
        todo!()
    }
    async fn shoutdown(&self, locator: &str) -> Result<bool, ()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn docker_runtime() {
        let runtime = DockerRuntime::new();

        assert!(runtime.is_ok());
    }
}
