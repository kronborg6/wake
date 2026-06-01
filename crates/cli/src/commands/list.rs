use common::{
    domain::container::Container, port::container::ContainerRuntime,
    service::container::ContainerService,
};

pub async fn ContainerList() -> Vec<Container> {
    todo!()
}

pub async fn findContainer(
    service: &ContainerService<dyn ContainerRuntime + Send + Sync>,
) -> Result<Container, ()> {
    let gg = service.list().await;
    todo!()
}
