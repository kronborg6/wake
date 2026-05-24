use std::sync::Arc;

use common::service::container::ContainerService;
use docker::DockerRuntime;

#[tokio::main]
async fn main() {
    let runtime = Arc::new(DockerRuntime::new().expect("no docker could be found"));

    let service = ContainerService::new(runtime);

    let containers = service.containers().await.unwrap();

    println!("{:?}", containers);
}
