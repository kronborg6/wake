use std::collections::HashMap;

use bollard::{Docker, plugin::ContainerSummary, query_parameters::InspectContainerOptions};
use common::domain::container::Container;
use common::error::container::ContainerError;

use crate::error::DockerError;

pub async fn get_all_containers(
    docker: &Docker,
    option: Option<HashMap<String, Vec<String>>>,
) -> Result<Vec<Container>, ContainerError> {
    let option = option.map(|filter| {
        bollard::query_parameters::ListContainersOptionsBuilder::default()
            .all(true)
            .filters(&filter)
            .build()
    });

    match docker
        .list_containers(
            option.or(Some(
                bollard::query_parameters::ListContainersOptionsBuilder::new()
                    .all(true)
                    .build(),
            )),
        )
        .await
    {
        Ok(v) => v
            .into_iter()
            .map(|s| DockerContainerSummary(s).try_into())
            .collect(),
        Err(_) => Err(ContainerError::CreateionError),
    }
}

pub async fn get_a_container(docker: &Docker) -> Option<ContainerSummary> {
    todo!()
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_all() {
        let docker = Docker::connect_with_local_defaults().unwrap();
        let con = get_all_containers(&docker, None).await;

        assert!(con.is_ok());

        if let Ok(vec) = con.as_ref() {
            assert!(!vec.is_empty());
        }

        println!("hello: {:?}", con);
    }
}
