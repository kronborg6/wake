use std::str::FromStr;

use crate::container::mapper::ContainerInspectResponseSummary;
use anyhow::{Context, Ok};
use bollard::Docker;
use bollard::plugin::{ContainerUpdateBody, RestartPolicy, RestartPolicyNameEnum};
use bollard::query_parameters::InspectContainerOptionsBuilder;

pub async fn update_container_restart_police(
    docker: &Docker,
    locator: &str,
    new_status: &str,
) -> anyhow::Result<ContainerInspectResponseSummary> {
    let config = ContainerUpdateBody {
        restart_policy: Some(RestartPolicy {
            name: Some(
                RestartPolicyNameEnum::from_str(new_status).unwrap_or(RestartPolicyNameEnum::NO),
            ),
            maximum_retry_count: None,
        }),
        ..Default::default()
    };

    docker
        .update_container(locator, config)
        .await
        .context("failed to update container restart status")?;

    let updated_container = docker
        .inspect_container(
            locator,
            Some(InspectContainerOptionsBuilder::default().size(true).build()),
        )
        .await
        .context("failed to fetched updated container")?;

    Ok(ContainerInspectResponseSummary(updated_container))
}
