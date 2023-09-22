use crate::config::PotFile;

/// Pot command handlers
pub struct Cmd;

impl Cmd {
    /// This sets up the initial state for the system to work.
    /// 1. Initialize the ~/pot/POT.toml file, creating it if it does
    /// not already exist.
    /// 2. Create the namespaces for the pots defined in ~/POT.toml
    /// 3. Setup the required directory structures for the pots and
    /// rootfs's for the different bowls.
    pub fn init() -> std::io::Result<()> {
        // Initialize the ~/pot/POT.toml file, creating it if it
        // does not already exist
        let pot_file = PotFile::init()?;

        Ok(())
    }
}
