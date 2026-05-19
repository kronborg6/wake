use bollard::Docker;
use bollard::errors::Error;
use bollard::plugin::{
    ContainerInspectResponse, ContainerSummary, ContainerUpdateBody, RestartPolicy,
    RestartPolicyNameEnum,
};
use bollard::query_parameters::InspectContainerOptionsBuilder;

/// Inspects a container and prints the full debug representation of the inspection response.
///
/// This helper takes a `(Docker, &ContainerSummary)` tuple, calls Docker's `inspect_container`
/// for the container ID found in `container.id`, and prints the returned `ContainerInspectResponse`
/// using the `{:?}` debug formatter.
///
/// # Panics
///
/// Panics if `container.id` is `None` or if the Docker inspection call returns an error (due to the use of `unwrap()`).
///
/// # Examples
///
/// ```no_run
/// use bollard::Docker;
/// use bollard::models::ContainerSummary;
///
/// async fn example() {
///     let docker = Docker::connect_with_socket_defaults().unwrap();
///     let container = ContainerSummary { id: Some("my-container-id".into()), ..Default::default() };
///     conc((docker, &container)).await;
/// }
/// ```
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

/// Inspect a container and return its detailed inspection response.
///
/// `id` may be a container ID or name.
///
/// # Returns
///
/// `Ok(ContainerInspectResponse)` containing the container's inspect data on success, or `Err(Error)` if the Docker API call fails.
///
/// # Examples
///
/// ```
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// use bollard::Docker;
/// // Create a Docker client using the platform-default socket.
/// let docker = Docker::connect_with_socket_defaults()?;
/// // Replace "my_container" with an actual container ID or name.
/// let resp = get_container(&docker, "my_container").await?;
/// println!("{}", resp.name.unwrap_or_default());
/// # Ok(()) }
/// ```
async fn get_container(
    docker: &bollard::Docker,
    id: &str,
) -> Result<ContainerInspectResponse, Error> {
    let options = InspectContainerOptionsBuilder::default().size(true).build();

    docker.inspect_container(id, Some(options)).await
}

/// Connects to the local Docker daemon, inspects a specific container, sets its restart policy to `Always`, and re-inspects to print the restart policy before and after the update.
///
/// The program uses a hard-coded container ID: it prints the container's current `HostConfig.restart_policy`,
/// updates the container's restart policy to `Always`, then re-inspects and prints the updated restart policy.
/// Errors are reported to standard error/output.
///
/// # Examples
///
/// ```no_run
/// // Run the compiled binary to perform the inspect-update-inspect flow against the hard-coded container.
/// // This example is `no_run` because it requires a local Docker daemon and the specific container ID.
/// fn main() {
///     // Execute the program (compiled binary) rather than calling the async entry directly.
///     // e.g., `cargo run --bin <binary-name>`
/// }
/// ```
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
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
// async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
//     let docker = Docker::connect_with_socket_defaults()?;
//
//     let options = InspectContainerOptionsBuilder::default().size(true).build();
//     let options2 = InspectContainerOptionsBuilder::default().size(true).build();
//
//     let container_id = "aeaf9a0b61c9bbebc09309d47088e8e33c74b14d2a50344f00d0efe5620a65cc";
//
//     let config = ContainerUpdateBody {
//         restart_policy: Some(RestartPolicy {
//             name: Some(RestartPolicyNameEnum::ALWAYS),
//             maximum_retry_count: None,
//         }),
//         ..Default::default()
//     };
//
//     match docker.inspect_container(container_id, Some(options)).await {
//         Ok(v) => {
//             let name = v
//                 .name
//                 .unwrap_or_default()
//                 .trim_start_matches('/')
//                 .to_string();
//
//             println!("Container name: {}", name);
//
//             match docker.update_container(&name, config).await {
//                 Ok(_) => println!("Container updated successfully"),
//                 Err(e) => {
//                     eprintln!("Error updating container {}: {:?}", name, e);
//                     return Ok(());
//                 }
//             }
//
//             // Re-inspect after update
//             let updated = docker.inspect_container(&name, Some(options2)).await?;
//
//             println!(
//                 "{}",
//                 serde_json::to_string_pretty(&updated.host_config.unwrap().restart_policy)?
//             );
//         }
//
//         Err(e) => println!("error: {:?}", e),
//     }
//
//     Ok(())
// }
