use crate::error::container::ContainerError;

#[derive(Debug, PartialEq, Eq)]
pub enum ContainerRestartPolicy {
    Empty,
    No,
    Always,
    UnlessStopped,
    OnFailure,
}

impl ContainerRestartPolicy {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContainerRestartPolicy::Empty => "",
            ContainerRestartPolicy::No => "no",
            ContainerRestartPolicy::Always => "always",
            ContainerRestartPolicy::UnlessStopped => "unless-stopped",
            ContainerRestartPolicy::OnFailure => "on-failure",
        }
    }
}

impl std::convert::From<&str> for ContainerRestartPolicy {
    fn from(name: &str) -> Self {
        match name {
            "no" => ContainerRestartPolicy::No,
            "always" => ContainerRestartPolicy::Always,
            "unless-stopped" => ContainerRestartPolicy::UnlessStopped,
            "on-failure" => ContainerRestartPolicy::OnFailure,
            _ => ContainerRestartPolicy::Empty,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Container {
    id: String,
    name: Vec<String>,
    restart_policy: ContainerRestartPolicy,
}

impl Container {
    pub fn new(
        id: String,
        name: Vec<String>,
        restart_policy: ContainerRestartPolicy,
    ) -> Result<Self, ContainerError> {
        if id.is_empty() || id.len() < 32 {
            return Err(ContainerError::MissingId);
        }
        Ok(Self {
            id,
            name,
            restart_policy,
        })
        // Self {
        //     id,
        //     name,
        //     restart_policy,
        // }
    }
    pub fn id(self) -> String {
        self.id
    }
    pub fn name(self) -> Vec<String> {
        self.name
    }
    pub fn restart_policy(self) -> ContainerRestartPolicy {
        self.restart_policy
    }
}
