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
    id: String,
    name: String,
    restart_policy: ContainerRestartPolicy,
}

impl Container {
    /// Creates a new `Container` with the given id, name, and restart policy.
    ///
    /// Validates that `id` and `name` are not empty; returns `Ok(Container)` on success or
    /// `Err(ContainerError::CreateionError)` if either `id` or `name` is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::domain::container::{Container, ContainerRestartPolicy};
    /// use crate::error::container::ContainerError;
    ///
    /// let c = Container::new(
    ///     "abc123".to_string(),
    ///     "my-container".to_string(),
    ///     ContainerRestartPolicy::ALWAYS,
    /// ).unwrap();
    /// assert_eq!(c.id(), "abc123");
    ///
    /// let err = Container::new("".to_string(), "name".to_string(), ContainerRestartPolicy::NO);
    /// assert!(matches!(err, Err(ContainerError::CreateionError)));
    /// ```
    pub fn new(
        id: String,
        name: String,
        restart_policy: ContainerRestartPolicy,
    ) -> Result<Self, ContainerError> {
        if id.is_empty() || name.is_empty() {
            return Err(ContainerError::CreateionError);
        }

        Ok(Self {
            id,
            name,
            restart_policy,
        })
    }

    /// Get the container's identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::domain::container::{Container, ContainerRestartPolicy};
    ///
    /// let c = Container::new("id".into(), "name".into(), ContainerRestartPolicy::EMPTY).unwrap();
    /// assert_eq!(c.id(), "id");
    /// ```
    ///
    /// # Returns
    ///
    /// A reference to the container's `id` (`&String`).
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get a reference to the container's name.
    ///
    /// # Returns
    ///
    /// A reference to the container's name.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::domain::container::{Container, ContainerRestartPolicy};
    ///
    /// let c = Container::new("id".to_string(), "web".to_string(), ContainerRestartPolicy::NO).unwrap();
    /// assert_eq!(c.name(), "web");
    /// ```
    pub fn name(&self) -> &String {
        &self.name
    }
    /// Consume the container and return its configured restart policy.
    ///
    /// # Returns
    ///
    /// `ContainerRestartPolicy` value representing the container's restart policy.
    ///
    /// # Examples
    ///
    /// ```
    /// let policy = ContainerRestartPolicy::ALWAYS;
    /// let container = Container::new("id".into(), "name".into(), policy).unwrap();
    /// let extracted = container.restart_policy();
    /// assert_eq!(extracted, ContainerRestartPolicy::ALWAYS);
    /// ```
    pub fn restart_policy(self) -> ContainerRestartPolicy {
        self.restart_policy
    }
}
