use crate::entities;

use dopamine_sdk::interfaces::engine;
use dopamine_sdk::math::{Mat3x4, Mat4x4, Vec3};
use dopamine_sdk::{Entity, Hitbox, WeaponId};

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
      let collideable = entity.collideable();

      self.entity_info.push(EntityInfo {
        obb_mins: *collideable.obb_mins(),
        obb_maxs: *collideable.obb_maxs(),
        coordinate_frame: entity.renderable().to_world_transform().clone(),
        extra_info: if entity.is_player() {
          PlayerInfo::new(entity).map(EntityInfoExtra::Player)
        } else {
          None
        },
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
pub struct PlayerInfo {
  pub health: i32,
  pub armor: i32,
  pub weapon: Option<WeaponInfo>,
  pub head_obb_mins: Vec3,
  pub head_obb_maxs: Vec3,
  pub head_to_world_transform: Mat3x4,
  pub is_dormant: bool,
  pub is_alive: bool,
  pub is_enemy: bool,
  pub is_defusing: bool,
  pub is_planting: bool,
  pub has_c4: bool,
  pub has_helmet: bool,
  pub has_defuser: bool,
}

impl PlayerInfo {
  fn new(entity: &Entity) -> Option<Self> {
    let (head_obb_mins, head_obb_maxs, head_to_world_matrix) = entity.bone_info(Hitbox::Head)?;
    let active_weapon = entity.active_weapon();

    Some(PlayerInfo {
      health: entity.health(),
      armor: entity.armor(),
      weapon: active_weapon.and_then(WeaponInfo::new),
      head_obb_mins,
      head_obb_maxs,
      head_to_world_transform: head_to_world_matrix.clone(),
      is_dormant: entity.networkable().is_dormant(),
      is_alive: entity.is_alive(),
      is_enemy: Entity::local_player().is_some_and(|lp| lp.team() != entity.team()),
      is_defusing: entity.is_defusing(),
      is_planting: entity.is_planting(),
      has_c4: entity.has_c4(),
      has_helmet: entity.has_helmet(),
      has_defuser: entity.has_defuser(),
    })
  }
}

#[derive(Debug)]
pub struct WeaponInfo {
  pub id: WeaponId,
  pub current_ammo: i32,
  pub max_ammo: i32,
  pub is_in_scope: bool,
}

impl WeaponInfo {
  fn new(entity: &Entity) -> Option<Self> {
    let (current_ammo, max_ammo) = entity.ammo()?;

    Some(Self { id: entity.weapon_id(), current_ammo, max_ammo, is_in_scope: entity.is_in_scope() })
  }
}

#[derive(Debug)]
pub struct EntityInfo {
  pub obb_mins: Vec3,
  pub obb_maxs: Vec3,
  pub coordinate_frame: Mat3x4,
  pub extra_info: Option<EntityInfoExtra>,
}

#[derive(Debug)]
pub enum EntityInfoExtra {
  Player(PlayerInfo),
}
