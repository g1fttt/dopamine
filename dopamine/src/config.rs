use crate::utils::Color;

use enum_map::{Enum, EnumArray, EnumMap};
use serde::{Deserialize, Serialize};
use strum::VariantNames;

use std::ops::{Deref, DerefMut};
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

#[derive(Clone, Copy, Enum, VariantNames, Serialize, Deserialize)]
pub enum GlowConfigKind {
  Enemies,
  Allies,
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

pub type GlowGroupConfig = EnumMapConfig<GlowConfigKind, GlowConfig>;

#[derive(Clone, Copy, Enum, VariantNames, Serialize, Deserialize)]
pub enum ChamsConfigKind {
  Enemies,
  Allies,
}

#[derive(Default, Clone, Copy, Enum, VariantNames, Serialize, Deserialize)]
#[repr(usize)]
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
pub struct ChamsConfig {
  #[serde(skip)]
  pub current_layer_index: usize,
  pub layers: [ChamsLayer; 2],
}

pub type ChamsGroupConfig = EnumMapConfig<ChamsConfigKind, ChamsConfig>;

#[derive(Serialize, Deserialize)]
pub struct EnumMapConfig<K, V>
where
  K: EnumArray<V> + EnumArray<Option<V>>,
  V: Default,
{
  #[serde(skip)]
  pub current_config_index: usize,
  #[serde(flatten)]
  inner: EnumMap<K, V>,
}

impl<K, V> Default for EnumMapConfig<K, V>
where
  K: EnumArray<V> + EnumArray<Option<V>>,
  V: Default,
{
  fn default() -> Self {
    Self {
      current_config_index: usize::default(),
      inner: Default::default(),
    }
  }
}

impl<K, V> Deref for EnumMapConfig<K, V>
where
  K: EnumArray<V> + EnumArray<Option<V>>,
  V: Default,
{
  type Target = EnumMap<K, V>;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl<K, V> DerefMut for EnumMapConfig<K, V>
where
  K: EnumArray<V> + EnumArray<Option<V>>,
  V: Default,
{
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}
