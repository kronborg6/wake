use std::str::FromStr;

use bollard::plugin::{ContainerUpdateBody, RestartPolicy, RestartPolicyNameEnum};
use bollard::query_parameters::InspectContainerOptionsBuilder;
use bollard::{Docker, plugin::ContainerInspectResponse};
use common::error::container::ContainerError;

use crate::container::mapper::ContainerInspectResponseSummary;
pub async fn update_container_restart_police(
    docker: &Docker,
    locator: &str,
    new_status: &str,
) -> Result<ContainerInspectResponseSummary, ContainerError> {
    let config = ContainerUpdateBody {
        restart_policy: Some(RestartPolicy {
            name: Some(
                RestartPolicyNameEnum::from_str(new_status).unwrap_or(RestartPolicyNameEnum::NO),
            ),
            maximum_retry_count: None,
        }),
        ..Default::default()
    };

    match docker.update_container(locator, config).await {
        Ok(_) => {
            match docker
                .inspect_container(
                    locator,
                    Some(InspectContainerOptionsBuilder::default().size(true).build()),
                )
                .await
            {
                Ok(u) => Ok(ContainerInspectResponseSummary(u)),
                Err(e) => Err(ContainerError::FetchingError(e.to_string())),
            }
        }
        Err(e) => {
            eprintln!("Error updating container {}: {:?}", locator, e);
            Err(ContainerError::FetchingError(e.to_string()))
        }
    }
}
