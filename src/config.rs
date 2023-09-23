use crate::{errors::PotError, pot::Pot};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// A Rust represantation of ~/pot/POT.toml
#[derive(Default, Deserialize, Serialize)]
pub struct PotFile {
    pub pot: Option<Vec<Pot>>,
}

impl PotFile {
    /// Get the ~/.pot/POT.toml file, creating it if it does not
    /// already exist
    pub fn init() -> std::io::Result<PotFile> {
        let mut home_dir = dirs::home_dir();

        if let Some(pot_file_path) = &mut home_dir {
            pot_file_path.push(".pot");

            // create ~/.pot directory if it does not exist
            if !pot_file_path.exists() {
                fs::create_dir(&pot_file_path)?;
            }

            // create ~/.pot/POT.toml if does not already exist
            let pot_file = PotFile::create_pot_file(pot_file_path);

            // create respective pot directories
            PotFile::create_pot_dirs(&pot_file);

            Ok(pot_file)
        } else {
            panic!("Could not open config directory")
        }
    }

    // Create potfile if it does not already exist
    fn create_pot_file(pot_file_path: &mut PathBuf) -> PotFile {
        // create potfile
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

        pot_file
    }

    // Create the respective pot directories in ~/.pot based on the
    // ~/.pot/POT.toml file
    fn create_pot_dirs(pot_file: &PotFile) {
        if let Some(pots) = &pot_file.pot {
            if let Some(pot_dir) = &mut dirs::home_dir() {
                // create ~/.pot/pots directory
                pot_dir.push(".pot");
                pot_dir.push("pots");
                if !pot_dir.exists() {
                    fs::create_dir(&pot_dir).expect("Could not create ~/.pot/pots directory");
                }

                for pot in pots {
                    // create the directory for the respective poterror
                    pot_dir.push(&pot.name);

                    if !pot_dir.exists() {
                        fs::create_dir(&pot_dir).expect("Could not create directory for pot")
                    }

                    // remember to pop the pot's name, to make way for the
                    // loops next iteration
                    pot_dir.pop();
                }
            } else {
                panic!("Could not locate home directory ~")
            }
        }
    }
}
