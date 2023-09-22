use crate::network::Network;
use serde::{Deserialize, Serialize};

/// __Process Organization Template__: A template used to organize a
/// set of processes.
#[derive(Default, Deserialize, Serialize)]
pub struct Pot {
    /// Unique name of the pot
    pub name: String,
    /// Base distro
    pub rootfs: String,
    /// Command to run when pot is opened
    pub open: String,
    /// Type of network to use for the pot
    pub network: Network,
}
