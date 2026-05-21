use common::service::container::ContainerService;
use docker::DockerRuntime;

#[tokio::main]
async fn main() {
    let runtime = DockerRuntime::new().unwrap();

    let service = ContainerService::new(runtime);

    let containers = service.containers().await.unwrap();

    println!("{:?}", containers);
}
