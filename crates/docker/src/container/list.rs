use std::collections::HashMap;

use bollard::plugin::ContainerInspectResponse;
use bollard::query_parameters::InspectContainerOptionsBuilder;
use bollard::{Docker, plugin::ContainerSummary};
use common::domain::container::Container;

use common::error::container::ContainerError;

use crate::container::mapper::DockerContainerSummary;

pub async fn get_all_containers(
    docker: &Docker,
    option: Option<&HashMap<String, Vec<String>>>,
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

pub async fn get_a_container(docker: &Docker, locator: &str) -> Option<ContainerInspectResponse> {
    let option = InspectContainerOptionsBuilder::default().size(true).build();

    docker.inspect_container(locator, Some(option)).await.ok()
}
