use std::collections::HashMap;

use bollard::plugin::ContainerInspectResponse;
use bollard::query_parameters::InspectContainerOptionsBuilder;
use bollard::{Docker, plugin::ContainerSummary};
use common::domain::container::Container;

use common::error::container::ContainerError;

use crate::container::mapper::{ContainerInspectResponseSummary, DockerContainerSummary};

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

pub async fn get_a_container(
    docker: &Docker,
    locator: &str,
) -> Result<ContainerInspectResponse, ContainerError> {
    let option = InspectContainerOptionsBuilder::default().size(true).build();

    match docker.inspect_container(locator, Some(option)).await {
        Ok(o) => Ok(o),
        Err(e) => Err(ContainerError::FetchingError(e.to_string())),
    }

    // ContainerInspectResponseSummary(container).try_into()
}
