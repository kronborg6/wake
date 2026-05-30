use anyhow::Ok;
use bollard::plugin::{ContainerInspectResponse, ContainerSummary, RestartPolicyNameEnum};
use common::{
    domain::container::{Container, ContainerRestartPolicy},
    error::container::ContainerError,
};

pub struct DockerContainerSummary(pub ContainerSummary);

pub struct ContainerInspectResponseSummary(pub ContainerInspectResponse);

impl TryInto<Container> for DockerContainerSummary {
    type Error = ContainerError;
    fn try_into(self) -> Result<Container, Self::Error> {
        Ok(Container::new(
            self.0.id.ok_or(ContainerError::MissingId)?,
            self.0.names.unwrap_or(vec![]),
            common::domain::container::ContainerRestartPolicy::Empty,
        )?)
        .map_err(|_| ContainerError::CreationError)
    }
}

impl TryInto<Container> for ContainerInspectResponseSummary {
    type Error = ContainerError;
    fn try_into(self) -> Result<Container, Self::Error> {
        let names = match self.0.name {
            Some(v) => vec![v],
            _ => vec![],
        };
        Ok(Container::new(
            self.0.id.ok_or(ContainerError::MissingId)?,
            names,
            self.0
                .host_config
                .and_then(|host_config| host_config.restart_policy)
                .and_then(|restart_policy| restart_policy.name)
                .map(|name| ContainerRestartPolicy::from(name.as_ref()))
                .unwrap_or(ContainerRestartPolicy::Empty),
        )?)
        .map_err(|_| ContainerError::CreationError)
    }
}

pub fn map_inspected_containers(
    containers: Vec<ContainerInspectResponse>,
) -> Result<Vec<Container>, ContainerError> {
    containers
        .into_iter()
        .map(|container| ContainerInspectResponseSummary(container).try_into())
        .collect()
}
