use std::collections::HashMap;

use bollard::{Docker, plugin::ContainerSummary, query_parameters::InspectContainerOptions};

use crate::error::DockerError;

pub async fn get_all_containers(
    docker: &Docker,
    option: Option<HashMap<String, Vec<String>>>,
) -> Result<Vec<ContainerSummary>, DockerError> {
    let option = option.map(|filter| {
        bollard::query_parameters::ListContainersOptionsBuilder::default()
            .all(true)
            .filters(&filter)
            .build()
    });

    docker
        .list_containers(option)
        .await
        .map_err(|_| DockerError::ConnectionError)
}

pub async fn get_a_container(docker: &Docker) -> Option<ContainerSummary> {
    todo!()
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
