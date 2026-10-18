use crate::features::chams::ChamsConfig;
use crate::features::esp::EspConfig;
use crate::features::glow::GlowConfig;
use crate::features::misc::MiscConfig;
use crate::features::visuals::VisualsConfig;

use educe::Educe;
use enum_map::{EnumArray, EnumMap};
use serde::{Deserialize, Serialize};
use strum::VariantArray;

use std::collections::HashSet;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::{fs, io};

#[derive(Educe, Serialize, Deserialize)]
#[serde(default)]
#[educe(Default)]
pub struct Config {
  pub misc: MiscConfig,
  pub visuals: VisualsConfig,
  pub glow: GlowConfig,
  pub chams: ChamsConfig,
  pub esp: EspConfig,

  #[educe(Default = true)]
  pub blur_enabled: bool,
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
    let pretty = yaml_serde::to_string(self)
      .inspect_err(|err| log::error!("Failed to serialize config as pretty string: {err}"))
      .unwrap();
    fs::write(path, pretty)
  }

  pub fn load_from<P>(&mut self, path: P) -> io::Result<()>
  where
    P: AsRef<Path>,
  {
    let raw = fs::read_to_string(path)?;
    *self = yaml_serde::from_str(&raw)
      .inspect_err(|err| log::error!("Failed to deserialize config file: {err}"))
      .unwrap();
    Ok(())
  }
}

#[derive(Serialize, Deserialize)]
pub struct MultiSelectConfig<E>
where
  E: VariantArray + Display + PartialEq,
{
  #[serde(default = "MultiSelectConfig::<E>::items", skip)]
  pub items: Vec<String>,
  pub selected: HashSet<usize>,
  #[serde(skip)]
  _e: PhantomData<E>,
}

impl<E> Default for MultiSelectConfig<E>
where
  E: VariantArray + Display + PartialEq,
{
  fn default() -> Self {
    Self { items: Self::items(), selected: HashSet::default(), _e: PhantomData }
  }
}

impl<E> MultiSelectConfig<E>
where
  E: VariantArray + Display + PartialEq,
{
  fn items() -> Vec<String> {
    E::VARIANTS.iter().map(|item| item.to_string()).collect()
  }

  pub fn is_selected(&self, item: &E) -> bool {
    E::VARIANTS
      .iter()
      .position(|item_| item_ == item)
      .is_some_and(|idx| self.selected.contains(&idx))
  }

  pub fn selected(&self) -> impl Iterator<Item = &str> {
    self
      .items
      .iter()
      .enumerate()
      .filter(|(i, _)| self.selected.contains(i))
      .map(|(_, s)| s.as_str())
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
