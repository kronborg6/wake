use bollard::plugin::{ContainerInspectResponse, ContainerSummary};
use common::{domain::container::Container, error::container::ContainerError};

pub struct DockerContainerSummary(pub ContainerSummary);

pub struct ContainerInspectResponseSummary(pub ContainerInspectResponse);

impl TryInto<Container> for DockerContainerSummary {
    type Error = ContainerError;
    fn try_into(self) -> Result<Container, Self::Error> {
        Ok(Container {
            id: self.0.id.ok_or(ContainerError::MissingId)?,
            name: self.0.names.unwrap_or(vec![]),
        })
    }
}

impl TryInto<Container> for ContainerInspectResponseSummary {
    type Error = ContainerError;
    fn try_into(self) -> Result<Container, Self::Error> {
        Ok(Container {
            id: self.0.id.ok_or(ContainerError::MissingId)?,
            name: match self.0.name {
                Some(v) => vec![v],
                _ => vec![],
            },
        })
    }
}
