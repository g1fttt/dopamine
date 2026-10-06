use crate::entities;

use dopamine_sdk::interfaces::{engine, model_info};
use dopamine_sdk::math::{Mat3x4, Mat4x4, Vec3};
use dopamine_sdk::{Entity, Hitbox};

#[derive(Debug, Default)]
pub struct GameState {
  view_matrix: Option<Mat4x4>,
  entity_info: Vec<EntityInfo>,
}

impl GameState {
  pub fn collect(&mut self) {
    self.entity_info.clear();
    self.view_matrix.replace(engine().world_to_screen_matrix().clone());

    for entity in entities::iter() {
      if entity.networkable().is_dormant() {
        continue;
      }

      let renderable = entity.renderable();

      let Some(header) = renderable.model().and_then(|mdl| model_info().get_studio_header(mdl))
      else {
        continue;
      };

      let Some(head_bbox) =
        header.hitbox_set(entity.hitbox_set() as usize).and_then(|set| set.hitbox(Hitbox::Head))
      else {
        continue;
      };

      let Some(head_to_world_matrix) = entity
        .bone_accessor()
        .and_then(|acc| acc.to_world_transform(head_bbox.bone_index as usize))
        .cloned()
      else {
        continue;
      };

      let collideable = entity.collideable();

      self.entity_info.push(EntityInfo {
        obb_mins: *collideable.obb_mins(),
        obb_maxs: *collideable.obb_maxs(),
        is_player: entity.is_player(),
        is_alive: entity.is_alive(),
        is_enemy: Entity::local_player().is_some_and(|lp| lp.team() != entity.team()),
        coordinate_frame: renderable.to_world_transform().clone(),

        head_obb_mins: head_bbox.mins,
        head_obb_maxs: head_bbox.maxs,
        head_to_world_matrix,
      });
    }
  }

  pub fn view_matrix(&self) -> Option<&Mat4x4> {
    self.view_matrix.as_ref()
  }

  pub fn entity_info(&self) -> &[EntityInfo] {
    &self.entity_info
  }
}

#[derive(Debug)]
pub struct EntityInfo {
  pub obb_mins: Vec3,
  pub obb_maxs: Vec3,
  pub is_player: bool,
  pub is_alive: bool,
  pub is_enemy: bool,
  pub coordinate_frame: Mat3x4,

  // TODO: Refactor
  pub head_obb_mins: Vec3,
  pub head_obb_maxs: Vec3,
  pub head_to_world_matrix: Mat3x4,
}
