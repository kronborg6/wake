use anyhow::Ok;
use bollard::plugin::ContainerInspectResponse;
use common::{
    domain::container::{Container, ContainerRestartPolicy, ContainerStateStatusEnum},
    error::container::ContainerError,
};

pub struct ContainerInspectResponseSummary(pub ContainerInspectResponse);

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
            self.0
                .state
                .and_then(|state| state.status)
                .map(|status| ContainerStateStatusEnum::from(status.as_ref()))
                .unwrap_or(ContainerStateStatusEnum::Empty),
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
