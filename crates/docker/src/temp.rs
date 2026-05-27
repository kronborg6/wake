use bollard::Docker;
use bollard::errors::Error;
use bollard::plugin::{
    ContainerInspectResponse, ContainerSummary, ContainerUpdateBody, RestartPolicy,
    RestartPolicyNameEnum,
};
use bollard::query_parameters::InspectContainerOptionsBuilder;

#[allow(dead_code)]
async fn conc(arg: (Docker, &ContainerSummary)) {
    let (docker, container) = arg;
    println!(
        "{:?}",
        docker
            .inspect_container(
                container.id.as_ref().unwrap(),
                None::<bollard::query_parameters::InspectContainerOptions>
            )
            .await
            .unwrap()
    )
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
//     let docker = Docker::connect_with_socket_defaults().unwrap();
//
//     let mut list_container_filters = HashMap::new();
//     list_container_filters.insert(String::from("status"), vec![String::from("running")]);
//
//     let containers = &docker
//         .list_containers(Some(
//             bollard::query_parameters::ListContainersOptionsBuilder::default()
//                 .all(true)
//                 .filters(&list_container_filters)
//                 .build(),
//         ))
//         .await?;
//
//     let docker_stream = stream::repeat(docker);
//     docker_stream
//         .zip(stream::iter(containers))
//         .for_each_concurrent(2, conc)
//         .await;
//
//     Ok(())
// }
//

#[allow(dead_code)]
async fn get_container(
    docker: &bollard::Docker,
    id: &str,
) -> Result<ContainerInspectResponse, Error> {
    let options = InspectContainerOptionsBuilder::default().size(true).build();

    docker.inspect_container(id, Some(options)).await
}

// #[tokio::main]
#[allow(dead_code)]
async fn maingg() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let docker = Docker::connect_with_socket_defaults().unwrap();

    let con = get_container(
        &docker,
        "aeaf9a0b61c9bbebc09309d47088e8e33c74b14d2a50344f00d0efe5620a65cc",
    );

    let config = ContainerUpdateBody {
        restart_policy: Some(RestartPolicy {
            name: Some(RestartPolicyNameEnum::ALWAYS),
            maximum_retry_count: None,
        }),
        ..Default::default()
    };

    match con.await {
        Ok(v) => {
            let name = v
                .name
                .unwrap_or_default()
                .trim_start_matches('/')
                .to_string();
            println!(
                "{}",
                serde_json::to_string_pretty(&v.host_config.unwrap().restart_policy).unwrap()
            );

            println!("{}", name);
            match docker.update_container(&name, config).await {
                Ok(_) => {
                    match get_container(
                        &docker,
                        "aeaf9a0b61c9bbebc09309d47088e8e33c74b14d2a50344f00d0efe5620a65cc",
                    )
                    .await
                    {
                        Ok(o) => {
                            println!(
                                "{}",
                                serde_json::to_string_pretty(
                                    &o.host_config.unwrap().restart_policy
                                )
                                .unwrap()
                            );
                        }
                        Err(e) => eprintln!("Error updating container {}: {:?}", name, e),
                    }
                }
                Err(e) => eprintln!("Error updating container {}: {:?}", name, e),
            }
        }
        Err(e) => println!("error: {:?}", e),
    }

    Ok(())
}
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let docker = Docker::connect_with_socket_defaults().unwrap();
//
//     loop {
//         let mut filter = HashMap::new();
//         filter.insert(String::from("status"), vec![String::from("running")]);
//         let containers = &docker
//             .list_containers(Some(
//                 bollard::query_parameters::ListContainersOptionsBuilder::default()
//                     .all(true)
//                     .filters(&filter)
//                     .build(),
//             ))
//             .await?;
//
//         if containers.is_empty() {
//             panic!("no running containers");
//         } else {
//             for container in containers {
//                 let container_id = container.id.as_ref().unwrap();
//                 let stream = &mut docker
//                     .stats(
//                         container_id,
//                         Some(
//                             bollard::query_parameters::StatsOptionsBuilder::default()
//                                 .stream(false)
//                                 .build(),
//                         ),
//                     )
//                     .take(1);
//
//                 while let Some(Ok(stats)) = stream.next().await {
//                     println!(
//                         "{} - {:?}: {:?} {:?}",
//                         container_id, &container.names, container.image, stats
//                     );
//                 }
//             }
//         }
//     }
// }
// #[tokio::main]
#[allow(dead_code)]
async fn main_temp() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let docker = Docker::connect_with_socket_defaults()?;

    let options = InspectContainerOptionsBuilder::default().size(true).build();
    let options2 = InspectContainerOptionsBuilder::default().size(true).build();

    let container_id = "aeaf9a0b61c9bbebc09309d47088e8e33c74b14d2a50344f00d0efe5620a65cc";

    let config = ContainerUpdateBody {
        restart_policy: Some(RestartPolicy {
            name: Some(RestartPolicyNameEnum::ALWAYS),
            maximum_retry_count: None,
        }),
        ..Default::default()
    };

    match docker.inspect_container(container_id, Some(options)).await {
        Ok(v) => {
            let name = v
                .name
                .unwrap_or_default()
                .trim_start_matches('/')
                .to_string();

            println!("Container name: {}", name);

            match docker.update_container(&name, config).await {
                Ok(_) => println!("Container updated successfully"),
                Err(e) => {
                    eprintln!("Error updating container {}: {:?}", name, e);
                    return Ok(());
                }
            }

            // Re-inspect after update
            let updated = docker.inspect_container(&name, Some(options2)).await?;

            println!(
                "{}",
                serde_json::to_string_pretty(&updated.host_config.unwrap().restart_policy)?
            );
        }

        Err(e) => println!("error: {:?}", e),
    }

    Ok(())
}
