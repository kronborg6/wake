use bollard::Docker as bDocker;

use crate::client::connect;

pub(crate) mod client;
pub mod container;
pub(crate) mod error;

struct Docker {
    docker: bDocker,
}

impl Docker {
    pub fn new() -> Self {
        let connection = connect().unwrap();
        Self { docker: connection }
    }
}
