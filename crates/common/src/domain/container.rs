use crate::error::container::ContainerError;

#[derive(Debug, PartialEq, Eq)]
pub enum ContainerRestartPolicy {
    Empty,
    No,
    Always,
    UnlessStopped,
    OnFailure,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ContainerStateStatusEnum {
    Empty,
    Created,
    Running,
    Paused,
    Restarting,
    Removing,
    Exited,
    Dead,
    Stopping,
}

impl std::convert::From<&str> for ContainerStateStatusEnum {
    fn from(name: &str) -> Self {
        match name {
            "created" => ContainerStateStatusEnum::Created,
            "running" => ContainerStateStatusEnum::Running,
            "paused" => ContainerStateStatusEnum::Paused,
            "restarting" => ContainerStateStatusEnum::Restarting,
            "removing" => ContainerStateStatusEnum::Removing,
            "exited" => ContainerStateStatusEnum::Exited,
            "dead" => ContainerStateStatusEnum::Dead,
            "stopping" => ContainerStateStatusEnum::Stopping,
            _ => ContainerStateStatusEnum::Empty,
        }
    }
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
    status: ContainerStateStatusEnum,
}

impl Container {
    pub fn new(
        id: String,
        name: Vec<String>,
        restart_policy: ContainerRestartPolicy,
        status: ContainerStateStatusEnum,
    ) -> Result<Self, ContainerError> {
        if id.is_empty() || id.len() == 63 {
            return Err(ContainerError::MissingId);
        }
        Ok(Self {
            id,
            name,
            restart_policy,
            status,
        })
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
    pub fn status(self) -> ContainerStateStatusEnum {
        self.status
    }
}
