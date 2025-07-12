use crate::features::chams::ChamsConfig;
use crate::features::glow::GlowConfig;
use crate::features::misc::MiscConfig;
use crate::features::model_changer::ModelChangerConfig;
use crate::features::visuals::VisualsConfig;

use educe::Educe;
use enum_map::{EnumArray, EnumMap};
use serde::{Deserialize, Serialize};

use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::{fs, io};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
  pub misc: MiscConfig,
  pub visuals: VisualsConfig,
  pub glow: GlowConfig,
  pub chams: ChamsConfig,
  pub model_changer: ModelChangerConfig,
}

impl Config {
  pub const PATH: &str = "dopamine/config.yaml";

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
    let pretty = serde_yaml_ng::to_string(self)
      .inspect_err(|err| log::error!("Failed to serialize config as pretty string: {err}"))
      .unwrap();
    fs::write(path, pretty)
  }

  pub fn load_from<P>(&mut self, path: P) -> io::Result<()>
  where
    P: AsRef<Path>,
  {
    let raw = fs::read_to_string(path)?;
    *self = serde_yaml_ng::from_str(&raw)
      .inspect_err(|err| log::error!("Failed to deserialize config file: {err}"))
      .unwrap();
    Ok(())
  }
}

#[derive(Educe, Serialize, Deserialize)]
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
