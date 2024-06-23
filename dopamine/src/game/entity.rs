use crate::interfaces::Interfaces;
use crate::patterns::Patterns;

use dopamine_macros::{netvar, virtual_method};

#[repr(C)]
pub struct UserCommand {
  pad: [u8; 36],
  pub buttons: i32,
}

impl UserCommand {
  pub const IN_JUMP: i32 = 1 << 1;
}

#[allow(dead_code)]
#[repr(C)]
pub enum WeaponId {
  Scout = 3,
  Aug = 8,
  SG550 = 13,
  Awp = 17,
  G3SG1 = 23,
  SG552 = 26,
}

#[repr(C)]
pub struct Entity;

impl Entity {
  const ON_GROUND: i32 = 1 << 0;

  #[inline]
  pub fn is_on_ground(&self) -> bool {
    (self.flags() & Self::ON_GROUND) != 0
  }

  #[inline]
  pub fn is_local_player(&self) -> bool {
    (Patterns::get().is_local_player)(self)
  }

  #[inline]
  pub fn attachments(&self) -> EntityAttachmentIterator {
    EntityAttachmentIterator::new(self)
  }

  pub fn is_sniper_rifle(&self) -> bool {
    use WeaponId::*;

    matches!(self.weapon_id(), Scout | Awp | G3SG1 | SG550)
  }

  pub fn is_rifle_with_scope(&self) -> bool {
    use WeaponId::*;

    self.is_sniper_rifle() || matches!(self.weapon_id(), Aug | SG552)
  }

  pub fn is_in_scope(&self) -> bool {
    self.is_rifle_with_scope() && self.weapon_mode() == 1 // Secondary
                                                          //
                                                          // enum with #[repr(C)] leading to crash
  }
}

impl Entity {
  #[virtual_method(index = 4)]
  fn networkable(&self) -> &NetworkableEntity;

  #[virtual_method(index = 5)]
  fn renderable(&self) -> &RenderableEntity;

  #[virtual_method(index = 131)]
  fn is_player(&self) -> bool;

  #[virtual_method(index = 222)]
  fn active_weapon(&self) -> Option<&Entity>;

  #[virtual_method(index = 365)]
  fn weapon_id(&self) -> WeaponId;

  #[netvar(path = "CBaseEntity->m_iTeamNum")]
  fn team(&self) -> i32;

  #[netvar(path = "CBasePlayer->m_fFlags")]
  fn flags(&self) -> i32;

  #[netvar(path = "CWeaponCSBase->m_weaponMode")]
  fn weapon_mode(&self) -> i32;
}

impl Entity {
  fn move_child(&self) -> Option<&Self> {
    let handle = unsafe { *(self as *const Self).byte_add(0x184).cast::<i32>() };
    Interfaces::get().entity_list.get_entity_from_handle(handle)
  }

  fn move_peer(&self) -> Option<&Self> {
    let handle = unsafe { *(self as *const Self).byte_add(0x188).cast::<i32>() };
    Interfaces::get().entity_list.get_entity_from_handle(handle)
  }
}

#[repr(C)]
pub struct NetworkableEntity;

impl NetworkableEntity {
  #[virtual_method(index = 8)]
  fn is_dormant(&self) -> bool;
}

#[repr(transparent)]
pub struct RenderableEntity(private::RenderableEntity);

impl RenderableEntity {
  #[inline]
  pub fn draw_model(&self) {
    self.as_ref().draw_model(1 /* StudioRender */);
  }
}

impl RenderableEntity {
  #[virtual_method(index = 3)]
  fn should_draw(&self) -> bool;
}

impl AsRef<private::RenderableEntity> for RenderableEntity {
  fn as_ref(&self) -> &private::RenderableEntity {
    &self.0
  }
}

pub struct EntityAttachmentIterator<'a> {
  entity: &'a Entity,
  first_pass: bool,
}

impl<'a> EntityAttachmentIterator<'a> {
  fn new(entity: &'a Entity) -> Self {
    Self {
      entity,
      first_pass: true,
    }
  }
}

impl<'a> Iterator for EntityAttachmentIterator<'a> {
  type Item = &'a Entity;

  fn next(&mut self) -> Option<Self::Item> {
    if self.first_pass {
      self.first_pass = false;
      self.entity = self.entity.move_child()?;
    } else {
      self.entity = self.entity.move_peer()?;
    }
    Some(self.entity)
  }
}

mod private {
  use dopamine_macros::virtual_method;

  #[repr(C)]
  pub struct RenderableEntity;

  impl RenderableEntity {
    #[virtual_method(index = 10)]
    fn draw_model(&self, flags: i32) -> i32;
  }
}
