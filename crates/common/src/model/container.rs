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
    pub name: String,
    pub restart_policy: ContainerRestartPolicy,
}
