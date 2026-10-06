use crate::entities;

use dopamine_sdk::Entity;
use dopamine_sdk::interfaces::engine;
use dopamine_sdk::math::{Mat3x4, Mat4x4, Vec3};

#[derive(Default)]
pub struct GameState<'a> {
  view_matrix: Option<&'a Mat4x4>,
  entity_info: Vec<EntityInfo<'a>>,
}

impl<'a> GameState<'a> {
  pub fn collect(&mut self) {
    self.entity_info.clear();
    self.view_matrix.replace(engine().world_to_screen_matrix());

    for entity in entities::iter() {
      if entity.networkable().is_dormant() {
        continue;
      }

      let collideable = entity.collideable();

      self.entity_info.push(EntityInfo {
        obb_mins: *collideable.obb_mins(),
        obb_maxs: *collideable.obb_maxs(),
        is_player: entity.is_player(),
        is_alive: entity.is_alive(),
        is_enemy: Entity::local_player().is_some_and(|lp| lp.team() != entity.team()),
        coordinate_frame: entity.renderable().to_world_transform(),
      });
    }
  }

  pub fn view_matrix(&self) -> Option<&'a Mat4x4> {
    self.view_matrix
  }

  pub fn entity_info(&self) -> &[EntityInfo<'a>] {
    &self.entity_info
  }
}

pub struct EntityInfo<'a> {
  pub obb_mins: Vec3,
  pub obb_maxs: Vec3,
  pub is_player: bool,
  pub is_alive: bool,
  pub is_enemy: bool,
  pub coordinate_frame: &'a Mat3x4,
}
