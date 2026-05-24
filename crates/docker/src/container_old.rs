// use bollard::{Docker, plugin::ContainerSummary};
// use common::domain::container::Container;
// use common::error::container::ContainerError;
//
// pub struct DockerContainerSummary(pub ContainerSummary);
//
// impl TryInto<Container> for DockerContainerSummary {
//     type Error = ContainerError;
//     fn try_into(self) -> Result<Container, Self::Error> {
//         Ok(Container {
//             id: self.0.id.ok_or(ContainerError::MissingId)?,
//             name: self.0.names.unwrap_or(vec![]),
//         })
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[tokio::test]
//     async fn test_all() {
//         let docker = Docker::connect_with_local_defaults().unwrap();
//         let con = get_all_containers(&docker, None).await;
//
//         assert!(con.is_ok());
//
//         if let Ok(vec) = con.as_ref() {
//             assert!(!vec.is_empty());
//         }
//
//         println!("hello: {:?}", con);
//     }
// }
