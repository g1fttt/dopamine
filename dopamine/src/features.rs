pub mod chams;
pub mod glow;
pub mod misc;
pub mod visuals;

use crate::App;

use dopamine_sdk::game::Entity;
use dopamine_sdk::Interfaces;

pub struct FeatureContext<'config, 'rest, T> {
  pub(self) interfaces: &'rest Interfaces<'rest>,
  pub(self) local_player: Option<&'rest Entity>,
  pub(self) config: &'config T,
}

impl<'config, 'rest, T> FeatureContext<'config, 'rest, T> {
  pub fn new(app: &App, config: &'config T) -> Self {
    Self { interfaces: Interfaces::get(), local_player: app.local_player, config }
  }
}
