use std::str::FromStr;

use anyhow::{Context, Ok};
use bollard::Docker;
use bollard::plugin::{ContainerUpdateBody, RestartPolicy, RestartPolicyNameEnum};

pub async fn update_container_restart_police(
    docker: &Docker,
    locator: &str,
    new_status: &str,
) -> anyhow::Result<()> {
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

    Ok(())
}
