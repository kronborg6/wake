use crate::{
    domain::container::Container, error::container::ContainerError,
    port::container::ContainerRuntime,
};
use std::pin::Pin;

pub struct ContainerService<R>
where
    R: ContainerRuntime,
{
    runtime: R,
}

impl<R> ContainerService<R>
where
    R: ContainerRuntime + std::marker::Sync,
{
    pub fn new(runtime: R) -> Self {
        Self { runtime }
    }
    pub fn containers<'service, 'future>(
        &'service self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Container>, ContainerError>> + Sync + Send + 'future>>
    where
        'service: 'future,
    {
        Box::pin(async move { self.runtime.containers().await })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::pin::Pin;

    struct MockRuntime;

    impl ContainerRuntime for MockRuntime {
        fn containers<'service, 'future>(
            &'service self,
        ) -> Pin<
            Box<
                dyn Future<Output = Result<Vec<Container>, ContainerError>> + Sync + Send + 'future,
            >,
        >
        where
            'service: 'future,
        {
            Box::pin(async move { Ok(vec![]) })
        }

        fn get<'service, 'locator, 'future>(
            &'service self,
            locator: &'locator str,
        ) -> Pin<Box<dyn Future<Output = Result<Option<Container>, ()>> + Send + 'future>>
        where
            'service: 'future,
            'locator: 'future,
            Self: 'future,
        {
            Box::pin(async move {
                if locator.is_empty() {
                    return Err(());
                }
                Ok(None)
            })
        }
        fn update_restart_policy<'a>(
            &'a self,
            locator: &'a str,
        ) -> Pin<Box<dyn Future<Output = Result<Option<Container>, ()>> + Send + 'a>> {
            todo!()
        }
        fn update_state<'a>(
            &'a self,
            locator: &'a str,
        ) -> Pin<Box<dyn Future<Output = Result<Container, ()>> + Send + 'a>> {
            todo!()
        }

        fn shoutdown<'a>(
            &'a self,
            locator: &'a str,
        ) -> Pin<Box<dyn Future<Output = Result<Option<bool>, ()>> + Send + 'a>> {
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
