use crate::error::container::{self, ContainerError};

#[derive(Debug, PartialEq, Eq)]
pub enum ContainerRestartPolicy {
    EMPTY,
    NO,
    ALWAYS,
    UNLESS_STOPPED,
    ON_FAILURE,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Container {
    pub id: String,
    pub name: Vec<String>,
    // restart_policy: ContainerRestartPolicy,
}

// impl Container {
//     pub fn new(
//         id: String,
//         name: String,
//         restart_policy: ContainerRestartPolicy,
//     ) -> Result<Self, ContainerError> {
//         if id.is_empty() || name.is_empty() {
//             return Err(ContainerError::CreateionError);
//         }
//
//         Ok(Self {
//             id,
//             name,
//             restart_policy,
//         })
//     }
//
//     pub fn id(&self) -> &String {
//         &self.id
//     }
//
//     pub fn name(&self) -> &String {
//         &self.name
//     }
//     pub fn restart_policy(&self) -> &ContainerRestartPolicy {
//         &self.restart_policy
//     }
// }
