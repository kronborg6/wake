use std::{env, sync::Arc};

use cli::run_cli;
use common::{domain::container::ContainerRestartPolicy, service::container::ContainerService};
use docker::DockerRuntime;

// #[tokio::main]
// async fn main() {
//     let runtime = Arc::new(DockerRuntime::new().expect("no docker could be found"));
//
//     let service = ContainerService::new(runtime);
//
//     let containers = service.list().await.unwrap();
//
//     for container in &containers {
//         println!("{:?}", container);
//     }
//
//     service
//         .update_restart_policy(
//             "aeaf9a0b61c9bbebc09309d47088e8e33c74b14d2a50344f00d0efe5620a65cc",
//             &ContainerRestartPolicy::Always,
//         )
//         .await
//         .unwrap();
// }

#[tokio::main]
async fn main() {
    let runtime = Arc::new(DockerRuntime::new().expect("no docker could be found"));

    let service = ContainerService::new(runtime);
    if env::args().len() > 1 {
        run_cli();
    } else {
        println!("tui");
    }
<<<<<<< HEAD
=======

    service
        .update_restart_policy(
            "149d78297df7f3174c03e0034684b37ee3aa30989ace7e73adb6770dac2ab68f",
            &ContainerRestartPolicy::No,
        )
        .await
        .unwrap();
>>>>>>> main
}
