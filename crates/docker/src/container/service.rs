use bollard::Docker as bDocker;
use common::domain::container::Container;
use common::domain::container::ContainerRestartPolicy;
use common::error::runtime::RuntimeError;
use common::port::container::ContainerRuntime;

use crate::container::mapper::ContainerInspectResponseSummary;
use crate::container::mapper::map_inspected_containers;
use crate::container::update::update_container_restart_police;
use crate::{client::connect, error::DockerError};
use std::collections::HashMap;
use std::pin::Pin;

use crate::container::list::{get_a_container, get_all_containers};
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
    fn containers<'service, 'filter, 'future>(
        &'service self,
        filter: Option<HashMap<String, Vec<String>>>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Container>, RuntimeError>> + Send + 'future>>
    where
        'service: 'future,
    {
        Box::pin(async move {
            get_all_containers(&self.docker, filter.as_ref())
                .await
                .map_err(|e| {
                    tracing::error!(error = ?e);
                    RuntimeError::Internal
                })
                .map(|s| map_inspected_containers(s).unwrap())
        })
    }
    fn get<'service, 'locator, 'future>(
        &'service self,
        locator: &'locator str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Container>, RuntimeError>> + Send + 'future>>
    where
        'service: 'future,
        'locator: 'future,
        Self: 'future,
    {
        Box::pin(async move {
            match get_a_container(&self.docker, locator).await {
                Ok(Some(o)) => {
                    let container: Container = ContainerInspectResponseSummary(o)
                        .try_into()
                        .map_err(|_| RuntimeError::MapError)?;

                    Ok(Some(container))
                }
                Ok(None) => Ok(None),
                Err(e) => {
                    tracing::error!(error = ?e);
                    Err(RuntimeError::Internal)
                }
            }
        })
    }
    fn update_restart_policy<'a>(
        &'a self,
        locator: &'a str,
        status: &'a ContainerRestartPolicy,
    ) -> Pin<Box<dyn Future<Output = Result<(), RuntimeError>> + Send + 'a>> {
        Box::pin(async move {
            match update_container_restart_police(&self.docker, locator, status.as_str()).await {
                Ok(_) => Ok(()),
                Err(e) => {
                    tracing::error!(error = ?e,
                        locator = locator,
                        "failed to map locator");
                    Err(RuntimeError::Internal)
                }
            }
        })
    }
    fn update_state<'a>(
        &'a self,
        locator: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Container>, RuntimeError>> + Send + 'a>> {
        todo!()
    }
    fn shoutdown<'a>(
        &'a self,
        locator: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<bool, RuntimeError>> + Send + 'a>> {
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
