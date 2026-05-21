use crate::{
    domain::container::Container, error::container::ContainerError,
    port::container::ContainerRuntime,
};

pub struct ContainerService<R>
where
    R: ContainerRuntime,
{
    runtime: R,
}

impl<R> ContainerService<R>
where
    R: ContainerRuntime,
{
    pub fn new(runtime: R) -> Self {
        Self { runtime }
    }
    pub async fn containers(&self) -> Result<Vec<Container>, ContainerError> {
        self.runtime.containers().await
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use super::*;

    struct MockRuntime;

    #[async_trait]
    impl ContainerRuntime for MockRuntime {
        async fn containers(&self) -> Result<Vec<Container>, ContainerError> {
            Ok(vec![])
        }
        async fn get(&self, locator: &str) -> Result<Option<Container>, ()> {
            if locator.is_empty() {
                return Err(());
            }
            Ok(None)
        }
        async fn update_restart_policy(&self, locator: &str) -> Result<Option<Container>, ()> {
            todo!()
        }
        async fn update_state(&self, locator: &str) -> Result<Option<Container>, ()> {
            todo!()
        }
        async fn shoutdown(&self, locator: &str) -> Result<bool, ()> {
            todo!()
        }
    }

    fn service() -> ContainerService<MockRuntime> {
        ContainerService::new(MockRuntime)
    }

    #[tokio::test]
    async fn get_by_emty_name() {
        let service = service();

        let result = service.runtime.get("").await;

        assert!(result.is_err());
    }
    #[tokio::test]
    async fn get_by_locator() {
        let service = service();

        let result = service.runtime.get("ASGLK").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn container() {
        let service = service();

        let result = service.runtime.containers().await;

        assert!(result.is_ok());
    }
}
