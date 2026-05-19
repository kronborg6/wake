use bollard::Docker as bDocker;

use crate::client::connect;

pub(crate) mod client;
pub mod container;
pub(crate) mod error;

struct Docker {
    docker: bDocker,
}

impl Docker {
    /// Creates a new Docker wrapper initialized with a Docker client.
    ///
    /// This constructs a `Docker` instance configured with the crate's Docker client.
    /// Panics if the underlying client connection cannot be established.
    ///
    /// # Examples
    ///
    /// ```
    /// let _docker = crate::Docker::new();
    /// ```
    pub fn new() -> Self {
        let connection = connect().unwrap();
        Self { docker: connection }
    }
}
