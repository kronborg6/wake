use std::collections::HashMap;

use bollard::{Docker, plugin::ContainerSummary, query_parameters::InspectContainerOptions};

use crate::error::DockerError;

pub async fn get_all_containers(docker: &Docker) -> Result<Vec<ContainerSummary>, DockerError> {
    // let mut list_container_filter = HashMap::new();
    // list_container_filter.insert(k, v)
    //

    docker
        .list_containers(None)
        .await
        .map_err(|_| DockerError::ConnectionError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_all() {
        let docker = Docker::connect_with_local_defaults().unwrap();
        let con = get_all_containers(&docker).await;

        assert!(con.is_ok());

        if let Ok(vec) = con.as_ref() {
            assert!(!vec.is_empty());
        }

        println!("hello: {:?}", con);
    }
}
