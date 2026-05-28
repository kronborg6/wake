use std::collections::HashMap;

use anyhow::Context;
use bollard::Docker;
use bollard::plugin::ContainerInspectResponse;
use bollard::query_parameters::InspectContainerOptionsBuilder;
use common::domain::container::Container;

use crate::container::mapper::DockerContainerSummary;

pub async fn get_all_containers(
    docker: &Docker,
    option: Option<&HashMap<String, Vec<String>>>,
) -> anyhow::Result<Vec<Container>> {
    let option = option.map(|filter| {
        bollard::query_parameters::ListContainersOptionsBuilder::default()
            .all(true)
            .filters(filter)
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
            .map(|s| {
                DockerContainerSummary(s)
                    .try_into()
                    .context("failed to convonte to DockerContainer")
            })
            .collect(),
        Err(e) => Err(anyhow::Error::new(e).context("failed to get list of containers")),
    }
}

pub async fn get_a_container(
    docker: &Docker,
    locator: &str,
) -> anyhow::Result<Option<ContainerInspectResponse>> {
    let option = InspectContainerOptionsBuilder::default().size(true).build();

    match docker.inspect_container(locator, Some(option)).await {
        Ok(container) => Ok(Some(container)),
        Err(bollard::errors::Error::DockerResponseServerError {
            status_code: 404,
            message,
        }) => {
            return Ok(None);
        }
        Err(err) => {
            return Err(err).context("failed to get container");
        }
    }
}
