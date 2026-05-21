use common::service::container::ContainerService;
use docker::DockerAPI;

#[tokio::main]
async fn main() {
    let runtime = DockerAPI::new().unwrap();

    let service = ContainerService::new(runtime);

    let containers = service.containers().await.unwrap();

    println!("{:?}", containers);
}
