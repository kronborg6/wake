use bollard::Docker as bDocker;
use common::port::container::ContainerRuntime;
use common::{domain::container::Container, error::container::ContainerError};

use crate::{client::connect, error::DockerError};
use std::pin::Pin;

use crate::container::list::get_all_containers;
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

impl ContainerRuntime for DockerRuntime {
    fn containers<'service, 'future>(
        &'service self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Container>, ContainerError>> + Send + 'future>>
    where
        'service: 'future,
    {
        Box::pin(async move { get_all_containers(&self.docker, None).await })
    }
    fn get<'service, 'locator, 'future>(
        &'service self,
        locator: &'locator str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Container>, ()>> + Send + 'future>>
    where
        'service: 'future,
        'locator: 'future,
        Self: 'future,
    {
        Box::pin(async move {
            if locator.is_empty() {
                return Err(());
            }
            todo!()
        })
    }
    fn update_restart_policy<'a>(
        &'a self,
        locator: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Container>, ()>> + Send + 'a>> {
        todo!()
    }
    fn update_state<'a>(
        &'a self,
        locator: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Container>, ()>> + Send + 'a>> {
        todo!()
    }
    fn shoutdown<'a>(
        &'a self,
        locator: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<bool, ()>> + Send + 'a>> {
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
