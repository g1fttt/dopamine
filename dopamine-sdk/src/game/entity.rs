use crate::game::{ClassId, ClientClass};
use crate::interfaces::{engine, entity_list};
use crate::utils::Patterns;

use dopamine_macros::{netvar, virtual_method};
use open_enum::open_enum;

use std::ops::BitAnd;

#[repr(C)]
pub struct Entity {
  pad: [u8; 0x200],
  move_child_handle: EntityHandle,
  move_peer_handle: EntityHandle,
}

impl Entity {
  #[inline]
  pub fn is_on_ground(&self) -> bool {
    !self.flags().have(EntityFlags::OnGround)
  }

  #[inline(always)]
  pub fn is_local_player(&self) -> bool {
    (Patterns::get().is_local_player)(self)
  }

  #[inline(always)]
  pub fn attachments(&self) -> EntityAttachmentIterator<'_> {
    EntityAttachmentIterator::new(self)
  }

  #[inline]
  pub fn is_viewmodel(&self) -> bool {
    self.networkable().client_class().id == ClassId::PredictedViewModel
  }

  #[inline]
  pub fn is_spotted(&self, index: usize) -> bool {
    self.player_spotted()[index]
  }

  pub fn is_sniper_rifle(&self) -> bool {
    matches!(self.weapon_id(), WeaponId::Scout | WeaponId::Awp | WeaponId::G3SG1 | WeaponId::SG550)
  }

  pub fn is_rifle_with_scope(&self) -> bool {
    self.is_sniper_rifle() || matches!(self.weapon_id(), WeaponId::Aug | WeaponId::SG552)
  }

  pub fn is_in_scope(&self) -> bool {
    self.is_rifle_with_scope() && self.weapon_mode() == WeaponMode::Secondary
  }

  pub fn is_weapon(&self) -> bool {
    matches!(
      self.networkable().client_class().id,
      ClassId::Ak47
        | ClassId::C4
        | ClassId::DEagle
        | ClassId::Aug
        | ClassId::AWP
        | ClassId::Elite
        | ClassId::Famas
        | ClassId::FiveSeven
        | ClassId::G3SG1
        | ClassId::Galil
        | ClassId::Glock
        | ClassId::M249
        | ClassId::M3
        | ClassId::M4A1
        | ClassId::Mac10
        | ClassId::Mp5N
        | ClassId::P228
        | ClassId::P90
        | ClassId::Scout
        | ClassId::Sg550
        | ClassId::Sg552
        | ClassId::Tmp
        | ClassId::Ump45
        | ClassId::Usp
        | ClassId::Xm1014
    )
  }

  #[inline(always)]
  pub fn local_player() -> Option<&'static Self> {
    entity_list().get_entity_by_index(engine().local_player_index())
  }

  #[inline(always)]
  fn move_child(&self) -> Option<&Self> {
    entity_list().get_entity_from_handle(&self.move_child_handle)
  }

  #[inline(always)]
  fn move_peer(&self) -> Option<&Self> {
    entity_list().get_entity_from_handle(&self.move_peer_handle)
  }
}

impl Entity {
  virtual_method!(pub fn networkable[4](&self) -> &NetworkableEntity);
  virtual_method!(pub fn renderable[5](&self) -> &RenderableEntity);
  virtual_method!(pub fn is_player[132](&self) -> bool);
  virtual_method!(pub fn active_weapon[227](&self) -> Option<&Entity>);
  virtual_method!(pub fn weapon_id[371](&self) -> WeaponId);

  netvar!(pub fn team -> i32 for CBaseEntity->m_iTeamNum);
  netvar!(pub fn owner_handle -> EntityHandle for CBaseCombatWeapon->m_hOwner);
  netvar!(fn player_spotted -> [bool; 65] for CCSPlayerResource->m_bPlayerSpotted);
  netvar!(fn flags -> EntityFlags for CBasePlayer->m_fFlags);
  netvar!(fn weapon_mode -> WeaponMode for CWeaponCSBase->m_weaponMode);
  netvar!(fn observer_target_handle -> EntityHandle for CBasePlayer->m_hObserverTarget);
}

#[repr(C)]
pub struct NetworkableEntity;

impl NetworkableEntity {
  virtual_method!(pub fn release[1](&self));
  virtual_method!(pub fn client_class[2](&self) -> &ClientClass<'_>);
  virtual_method!(pub fn is_dormant[8](&self) -> bool);
  virtual_method!(pub fn index[9](&self) -> i32);
}

#[repr(C)]
pub struct RenderableEntity;

impl RenderableEntity {
  virtual_method!(pub fn should_draw[3](&self) -> bool);
  virtual_method!(pub fn draw_model[10](&self) -> i32 where (1: i32 /* StudioRender */));
}

#[repr(C)]
pub struct UserCommand {
  pad: [u8; 40],
  pub buttons: i32,
}

impl UserCommand {
  pub const IN_JUMP: i32 = 1 << 1;
}

#[derive(Clone, Copy)]
#[open_enum]
#[repr(C)]
enum EntityFlags {
  OnGround = 1 << 0,
}

impl EntityFlags {
  #[inline(always)]
  fn have(self, flags: EntityFlags) -> bool {
    (self & flags) == 0
  }
}

impl BitAnd for EntityFlags {
  type Output = i32;

  #[inline(always)]
  fn bitand(self, rhs: Self) -> Self::Output {
    self.0 & rhs.0
  }
}

#[open_enum]
#[derive(Clone, Copy, Hash, Debug)]
#[repr(C)]
pub enum WeaponId {
  Glock = 2,
  Scout,
  Aug = 8,
  SG550 = 13,
  Awp = 17,
  G3SG1 = 23,
  SG552 = 26,
  AK47,
}

#[derive(Clone, Copy)]
#[open_enum]
#[repr(C)]
enum WeaponMode {
  Secondary = 1,
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
