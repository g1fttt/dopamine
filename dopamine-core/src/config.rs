use serde::{Deserialize, Serialize};

use std::path::Path;
use std::{fs, io};

impl Config {
    pub const PATH: &'static str = "dopamine.toml";

    pub fn create_and_load_from<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let mut this = Config::default();
        // We defaulted config just now, so it doesn't matter if config file isn't exist
        this.load_from(path).ok();
        this
    }

    pub fn save_to<P>(&self, path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let pretty =
            toml::to_string_pretty(self).expect("Failed to serialize config as pretty TOML string");
        fs::write(path, pretty)
    }

    pub fn load_from<P>(&mut self, path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let raw = fs::read_to_string(path)?;
        *self = toml::from_str(&raw).expect("Failed to deserialize config file");
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct BunnyhopConfig {
    pub enabled: bool,
    pub chance: u8,
}

impl Default for BunnyhopConfig {
    fn default() -> Self {
        Self {
            enabled: bool::default(),
            chance: 100,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct MiscConfig {
    pub bunnyhop: BunnyhopConfig,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub misc: MiscConfig,
}
