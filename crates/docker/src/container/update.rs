use bollard::{Docker, plugin::ContainerInspectResponse};
use common::error::container::ContainerError;

pub async fn update_container_restart_police(
    docker: &Docker,
    locator: &str,
) -> Result<ContainerInspectResponse, ContainerError> {
    todo!()
}
