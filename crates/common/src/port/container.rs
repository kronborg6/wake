use crate::{domain::container::Container, error::container::ContainerError};
use async_trait::async_trait;

#[async_trait]
pub trait ContainerRuntime {
    async fn containers(&self) -> Result<Vec<Container>, ContainerError>;
    async fn get(&self, locator: &str) -> Result<Option<Container>, ()>;
    async fn update_restart_policy(&self, locator: &str) -> Result<Option<Container>, ()>;
    async fn update_state(&self, locator: &str) -> Result<Option<Container>, ()>;
    async fn shoutdown(&self, locator: &str) -> Result<bool, ()>;
    // async fn containers(&self) -> Result<Vec<Container>, ()>;
}
