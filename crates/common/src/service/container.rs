use crate::{
    domain::container::Container, error::container::ContainerError,
    port::container::ContainerRuntime,
};
use std::{pin::Pin, sync::Arc};

pub struct ContainerService<R>
where
    R: ContainerRuntime,
{
    runtime: Arc<R>,
}

impl<R> ContainerService<R>
where
    R: ContainerRuntime + std::marker::Sync + std::marker::Send,
{
    pub fn new(runtime: Arc<R>) -> Self {
        Self { runtime }
    }
    pub fn containers<'service, 'future>(
        &'service self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Container>, ContainerError>> + Send + 'future>>
    where
        'service: 'future,
    {
        Box::pin(async move {
            self.runtime
                .containers(None)
                .await
                .map_err(|error| ContainerError::FetchingError(error.to_string()))
        })
    }
    pub fn get<'service, 'locator, 'future>(
        &'service self,
        locator: &'locator str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Container>, ContainerError>> + Send + 'future>>
    where
        'service: 'future,
        'locator: 'future,
        Self: 'future,
    {
        Box::pin(async move {
            if locator.is_empty() {
                return Err(ContainerError::InvaldeLocator);
            }
            self.runtime
                .get(locator)
                .await
                .map_err(|error| ContainerError::FetchingError(error.to_string()))
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::error::runtime::RuntimeError;

    use super::*;
    use std::{collections::HashMap, pin::Pin};

    struct MockRuntime;

    impl ContainerRuntime for MockRuntime {
        fn containers<'service, 'future>(
            &'service self,
            _filter: Option<HashMap<String, Vec<String>>>,
        ) -> Pin<Box<dyn Future<Output = Result<Vec<Container>, RuntimeError>> + Send + 'future>>
        where
            'service: 'future,
        {
            Box::pin(async move { Ok(vec![]) })
        }

        fn get<'service, 'locator, 'future>(
            &'service self,
            _locator: &'locator str,
        ) -> Pin<Box<dyn Future<Output = Result<Option<Container>, RuntimeError>> + Send + 'future>>
        where
            'service: 'future,
            'locator: 'future,
            Self: 'future,
        {
            Box::pin(async move { Ok(None) })
        }
        fn update_restart_policy<'a>(
            &'a self,
            _locator: &'a str,
        ) -> Pin<Box<dyn Future<Output = Result<Option<Container>, RuntimeError>> + Send + 'a>>
        {
            todo!()
        }
        fn update_state<'a>(
            &'a self,
            _locator: &'a str,
        ) -> Pin<Box<dyn Future<Output = Result<Option<Container>, RuntimeError>> + Send + 'a>>
        {
            todo!()
        }

        fn shoutdown<'a>(
            &'a self,
            _locator: &'a str,
        ) -> Pin<Box<dyn Future<Output = Result<bool, RuntimeError>> + Send + 'a>> {
            todo!()
        }
    }

    fn service() -> ContainerService<MockRuntime> {
        ContainerService::new(Arc::new(MockRuntime))
    }

    #[tokio::test]
    async fn get_by_emty_name() {
        let service = service();

        let result = service.get("").await;

        assert!(result.is_err());
    }
    #[tokio::test]
    async fn get_by_vailed_locator() {
        let service = service();

        let result = service.get("ASGLK").await;

        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn get_by_invailed_locator() {
        let service = service();

        let result = service.runtime.get("").await;

        assert!(result.is_err());
    }
    #[tokio::test]
    async fn container() {
        let service = service();

        let result = service.runtime.containers(None).await;

        assert!(result.is_ok());
    }
}
