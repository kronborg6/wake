use crate::{
    domain::container::Container, error::container::ContainerError, port::container::DockerApi,
};

pub struct ContainerService<R>
where
    R: DockerApi,
{
    runtime: R,
}

impl<R> ContainerService<R>
where
    R: DockerApi,
{
    pub fn new(runtime: R) -> Self {
        Self { runtime }
    }
    pub async fn containers(&self) -> Result<Vec<Container>, ContainerError> {
        self.runtime.containers().await
    }
}
