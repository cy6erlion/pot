use crate::pot::Pot;
use serde::{Deserialize, Serialize};
use std::fs;

/// A Rust represantation of ~/pot/POT.toml
#[derive(Default, Deserialize, Serialize)]
pub struct PotFile {
    pub pots: Option<Vec<Pot>>,
}

impl PotFile {
    /// Get the ~/.pot/POT.toml file, creating it if it does not
    /// already exist
    pub fn init() -> std::io::Result<PotFile> {
        let mut home_dir = dirs::home_dir();

        if let Some(pot_file_path) = &mut home_dir {
            pot_file_path.push(".pot");

            // create directory if it does not exist
            if !pot_file_path.exists() {
                fs::create_dir(&pot_file_path)?;
            }

            pot_file_path.push("POT.toml");

            let mut pot_file: PotFile = Default::default();

            if pot_file_path.exists() {
                let s = fs::read_to_string(&pot_file_path)
                    .expect("Could not read ~/.pot/POT.toml config file");
                pot_file = toml::from_str(&s).unwrap();
            } else {
                let ts = toml::to_string(&pot_file).unwrap();
                fs::write(pot_file_path, ts.as_bytes()).expect("Could not save ~/.pot/POT.toml");
            }

            Ok(pot_file)
        } else {
            panic!("Could not open config directory")
        }
    }
}
