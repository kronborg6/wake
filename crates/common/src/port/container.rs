use std::{collections::HashMap, pin::Pin};

use crate::{
    domain::container::{Container, ContainerRestartPolicy},
    error::{container::ContainerError, runtime::RuntimeError},
};
// use async_trait::async_trait;

// #[async_trait] Result<Vec<Container>, ContainerError>
pub trait ContainerRuntime {
    fn containers<'service, 'filter, 'future>(
        &'service self,
        filter: Option<HashMap<String, Vec<String>>>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Container>, RuntimeError>> + Send + 'future>>
    where
        'service: 'future;

    fn get<'service, 'locator, 'future>(
        &'service self,
        locator: &'locator str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Container>, RuntimeError>> + Send + 'future>>
    where
        'service: 'future,
        'locator: 'future,
        Self: 'future;

    fn update_restart_policy<'a>(
        &'a self,
        locator: &'a str,
        status: &'a ContainerRestartPolicy,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Container>, RuntimeError>> + Send + 'a>>;

    fn update_state<'a>(
        &'a self,
        locator: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Container>, RuntimeError>> + Send + 'a>>;
    fn shoutdown<'a>(
        &'a self,
        locator: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<bool, RuntimeError>> + Send + 'a>>;
}
