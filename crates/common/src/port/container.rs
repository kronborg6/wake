use std::pin::Pin;

use crate::{domain::container::Container, error::container::ContainerError};
// use async_trait::async_trait;

// #[async_trait] Result<Vec<Container>, ContainerError>
pub trait ContainerRuntime {
    fn containers<'service, 'future>(
        &'service self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Container>, ContainerError>> + Send + 'future>>
    where
        'service: 'future;

    fn get<'service, 'locator, 'future>(
        &'service self,
        locator: &'locator str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Container>, ()>> + Send + 'future>>
    where
        'service: 'future,
        'locator: 'future,
        Self: 'future;

    fn update_restart_policy<'a>(
        &'a self,
        locator: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Container>, ()>> + Send + 'a>>;

    fn update_state<'a>(
        &'a self,
        locator: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Container>, ()>> + Send + 'a>>;
    fn shoutdown<'a>(
        &'a self,
        locator: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<bool, ()>> + Send + 'a>>;
}
