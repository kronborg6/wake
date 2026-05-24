use bollard::plugin::ContainerSummary;
use common::{domain::container::Container, error::container::ContainerError};

pub struct DockerContainerSummary(pub ContainerSummary);

impl TryInto<Container> for DockerContainerSummary {
    type Error = ContainerError;
    fn try_into(self) -> Result<Container, Self::Error> {
        Ok(Container {
            id: self.0.id.ok_or(ContainerError::MissingId)?,
            name: self.0.names.unwrap_or(vec![]),
        })
    }
}
