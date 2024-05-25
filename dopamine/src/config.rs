use crate::utils::Color;

use enum_map::{Enum, EnumMap};
use serde::{Deserialize, Serialize};

use std::path::Path;
use std::{fs, io};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub misc: MiscGroupConfig,
    pub glow: GlowGroupConfig,
    pub chams: ChamsGroupConfig,
}

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
pub struct MiscGroupConfig {
    pub bunnyhop: BunnyhopConfig,
}

#[derive(Serialize, Deserialize)]
pub struct GlowConfig {
    pub enabled: bool,
    pub color: Color,
}

impl Default for GlowConfig {
    fn default() -> Self {
        Self {
            enabled: bool::default(),
            color: Color::white(),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct GlowGroupConfig {
    pub enemies: GlowConfig,
    pub allies: GlowConfig,
}

#[derive(Enum, Serialize, Deserialize)]
pub enum ChamsConfigKind {
    Enemies,
    Allies,
}

#[derive(Default, Clone, Copy, Enum, Serialize, Deserialize)]
pub enum ChamsKind {
    #[default]
    Regular,
    Flat,
}

#[derive(Serialize, Deserialize)]
pub struct ChamsLayer {
    pub enabled: bool,
    pub ignore_z: bool,
    pub material_kind: ChamsKind,
    pub material_color: Color,
}

impl Default for ChamsLayer {
    fn default() -> Self {
        Self {
            enabled: bool::default(),
            ignore_z: bool::default(),
            material_kind: ChamsKind::default(),
            material_color: Color::white(),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct ChamsGroupConfig(pub EnumMap<ChamsConfigKind, [ChamsLayer; 2]>);
