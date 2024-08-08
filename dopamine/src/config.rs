use dopamine_utils::Color;
use educe::Educe;
use enum_map::{Enum, EnumArray, EnumMap};
use serde::{Deserialize, Serialize};
use strum::VariantNames;

use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::{fs, io};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
  pub misc: MiscGroupConfig,
  pub visuals: VisualsGroupConfig,
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
    let _ = this.load_from(path);
    this
  }

  pub fn save_to<P>(&self, path: P) -> io::Result<()>
  where
    P: AsRef<Path>,
  {
    let pretty = toml::to_string_pretty(self).expect("Failed to serialize config as pretty string");
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

#[derive(Educe, Serialize, Deserialize)]
#[serde(default)]
#[educe(Default)]
pub struct BunnyhopConfig {
  pub enabled: bool,
  #[educe(Default = 100)]
  pub chance: u8,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MiscGroupConfig {
  pub bunnyhop: BunnyhopConfig,
}

#[derive(Educe, Serialize, Deserialize)]
#[serde(default)]
#[educe(Default)]
pub struct NoScopeCrosshairConfig {
  pub enabled: bool,
  #[educe(Default = 5.0)]
  pub size: f32,
  #[educe(Default = 1.0)]
  pub thickness: f32,
  #[educe(Default = Color::white())]
  pub color: Color,
}

#[derive(Educe, Serialize, Deserialize)]
#[serde(default)]
#[educe(Default)]
pub struct AddFovConfig {
  pub enabled: bool,
  #[educe(Default = 10.0)]
  pub amount: f32,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct VisualsGroupConfig {
  pub no_scope_crosshair: NoScopeCrosshairConfig,
  pub add_fov: AddFovConfig,
}

#[derive(Clone, Copy, Enum, VariantNames, Serialize, Deserialize)]
pub enum GlowConfigKind {
  Enemies,
  Allies,
}

#[derive(Educe, Serialize, Deserialize)]
#[serde(default)]
#[educe(Default)]
pub struct GlowConfig {
  pub enabled: bool,
  #[educe(Default = Color::white())]
  pub color: Color,
}

pub type GlowGroupConfig = EnumMapConfig<GlowConfigKind, GlowConfig>;

#[derive(Clone, Copy, Enum, VariantNames, Serialize, Deserialize)]
pub enum ChamsConfigKind {
  Enemies,
  Allies,
  Viewmodel,
}

#[derive(Default, Clone, Copy, Enum, VariantNames, Serialize, Deserialize)]
#[repr(usize)] // Guarantee for `mem::transmute` in `ui::menu`
pub enum ChamsKind {
  #[default]
  Regular,
  Flat,
}

#[derive(Educe, Serialize, Deserialize)]
#[serde(default)]
#[educe(Default)]
pub struct ChamsLayerConfig {
  pub enabled: bool,
  pub ignore_z: bool,
  pub wireframe: bool,
  pub cover: bool,
  pub material_kind: ChamsKind,
  #[educe(Default = Color::white())]
  pub material_color: Color,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ChamsConfig {
  #[serde(skip)]
  pub current_layer_index: usize,
  pub layers: [ChamsLayerConfig; 4],
}

pub type ChamsGroupConfig = EnumMapConfig<ChamsConfigKind, ChamsConfig>;

#[derive(Educe, Serialize, Deserialize)]
#[serde(default)]
#[educe(Default)]
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
