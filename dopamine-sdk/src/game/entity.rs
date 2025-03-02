use super::{ClientClass, WeaponClassId};

use crate::utils::{Interfaces, Patterns};

use dopamine_macros::{netvar, virtual_method};

#[repr(C)]
pub struct UserCommand {
  pad: [u8; 40],
  pub buttons: i32,
}

impl UserCommand {
  pub const IN_JUMP: i32 = 1 << 1;
}

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
pub struct Entity {
  pad: [u8; 0x200],
  move_child_handle: EntityHandle,
  move_peer_handle: EntityHandle,
}

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

  #[inline(always)]
  pub fn attachments(&self) -> EntityAttachmentIterator {
    EntityAttachmentIterator::new(self)
  }
}

impl Entity {
  pub fn is_viewmodel(&self) -> bool {
    self.networkable().client_class().id == 89 /* CPredictedViewModel */
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

  pub fn is_weapon(&self) -> bool {
    WeaponClassId::from_repr(self.networkable().client_class().id as usize).is_some()
  }
}

impl Entity {
  virtual_method!(pub fn networkable[4](&self) -> &NetworkableEntity);
  virtual_method!(pub fn renderable[5](&self) -> &RenderableEntity);
  virtual_method!(pub fn is_player[132](&self) -> bool);
  virtual_method!(pub fn active_weapon[227](&self) -> Option<&Entity>);

  netvar!(pub fn team -> i32 for CBaseEntity->m_iTeamNum);
  netvar!(pub fn owner_handle -> EntityHandle for CBaseCombatWeapon->m_hOwner);
}

impl Entity {
  virtual_method!(fn weapon_id[371](&self) -> WeaponId);

  netvar!(fn flags -> i32 for CBasePlayer->m_fFlags);
  netvar!(fn weapon_mode -> i32 for CWeaponCSBase->m_weaponMode);
}

impl Entity {
  #[inline]
  fn move_child(&self) -> Option<&Self> {
    Interfaces::get().entity_list.get_entity_from_handle(&self.move_child_handle)
  }

  #[inline]
  fn move_peer(&self) -> Option<&Self> {
    Interfaces::get().entity_list.get_entity_from_handle(&self.move_peer_handle)
  }
}

#[repr(C)]
pub struct NetworkableEntity;

impl NetworkableEntity {
  virtual_method!(pub fn is_dormant[8](&self) -> bool);
  virtual_method!(pub fn client_class[2](&self) -> &ClientClass);
}

#[repr(C)]
pub struct RenderableEntity;

impl RenderableEntity {
  virtual_method!(pub fn should_draw[3](&self) -> bool);
  virtual_method!(pub fn draw_model[10](&self) -> i32 where (1: i32 /* StudioRender */));
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct EntityHandle(u32);

impl EntityHandle {
  #[inline]
  pub fn is_invalid(self) -> bool {
    self.0 == u32::MAX
  }
}

pub struct EntityAttachmentIterator<'a> {
  entity: &'a Entity,
  first_pass: bool,
}

impl<'a> EntityAttachmentIterator<'a> {
  fn new(entity: &'a Entity) -> Self {
    Self { entity, first_pass: true }
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
