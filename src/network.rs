use crate::errors::PotError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Various configurations for a pot
#[derive(Deserialize, Serialize)]
pub enum Network {
    /// No networking is enabled for the pot
    None,
    /// Creates a veth pair for the pot, one end is in the pot and the
    /// other end is in the host
    Host,
    /// A single bridge (pot0) is created on the host, the respective
    /// veth ends for the pots are hooked to this bridge
    Bridge,
}

impl Default for Network {
    fn default() -> Self {
        Network::Host
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let net = match self {
            Network::None => "None",
            Network::Host => "Host",
            Network::Bridge => "Bridge",
        };
        writeln!(f, "{net}")
    }
}

impl FromStr for Network {
    type Err = PotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Network::None),
            "Host" => Ok(Network::Host),
            "Bridge" => Ok(Network::Bridge),
            _ => Err(PotError::NetworkDeserializeError),
        }
    }
}
