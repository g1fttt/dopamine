use crate::cstr;
use crate::math::Vector3D;
use crate::utils::{Interfaces, Patterns};

use crate::game::engine::Model;
use crate::game::{ClassId, ClientClass};

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
  pub fn create_by_name(name: &str) -> Option<&'static Self> {
    (Patterns::get().create_entity_by_name)(cstr!(name))
  }
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
  pub fn set_model_index(&self, index: i32) {
    (Patterns::get().set_model_index)(self, index);
  }

  #[inline(always)]
  pub fn follow_entity(&self, parent: &Entity, bone_merge: bool) {
    (Patterns::get().follow_entity)(self, parent, bone_merge);
  }

  #[inline(always)]
  pub fn set_abs_origin(&self, origin: &Vector3D) {
    (Patterns::get().set_abs_origin)(self, origin);
  }

  #[inline(always)]
  pub fn lookup_sequence(&self, label: &str) -> i32 {
    (Patterns::get().lookup_sequence)(self, cstr!(label))
  }

  #[inline(always)]
  pub fn attachments(&self) -> EntityAttachmentIterator {
    EntityAttachmentIterator::new(self)
  }

  #[inline]
  pub fn viewmodel(&self) -> Option<&Self> {
    Interfaces::get().entity_list.get_entity_from_handle(&self.viewmodel_handle())
  }

  #[inline(always)]
  pub fn sequence_activity(&self, sequence: i32) -> i32 {
    (Patterns::get().get_sequence_activity)(self, sequence)
  }

  pub fn view_entity(&self) -> Option<&Self> {
    assert!(self.is_local_player());

    self.is_alive().then_some(self).or_else(|| self.observer_target())
  }

  // #[allow(clippy::mut_from_ref)]
  // pub fn client_side_animation(&self) -> &mut bool {
  //   unsafe { &mut *(self as *const Self).cast_mut().byte_add(0xAA0).cast::<bool>() }
  // }

  #[inline]
  pub fn is_viewmodel(&self) -> bool {
    self.networkable().client_class().id == ClassId::PredictedViewModel
  }

  #[inline]
  pub fn is_spotted(&self, index: usize) -> bool {
    self.player_spotted()[index]
  }

  #[inline]
  pub fn is_knife(&self) -> bool {
    self.weapon_id() == WeaponId::Knife
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
        | ClassId::Knife
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

  // #[allow(clippy::mut_from_ref)]
  // fn studio_header(&self) -> &mut StudioHeader {
  //   unsafe { &mut *(self as *const Self).cast_mut().byte_add(3000).cast::<StudioHeader>() }
  // }

  #[inline]
  fn observer_target(&self) -> Option<&Self> {
    Interfaces::get().entity_list.get_entity_from_handle(&self.observer_target_handle())
  }

  #[inline]
  fn move_child(&self) -> Option<&Self> {
    Interfaces::get().entity_list.get_entity_from_handle(&self.move_child_handle)
  }

  #[inline]
  fn move_peer(&self) -> Option<&Self> {
    Interfaces::get().entity_list.get_entity_from_handle(&self.move_peer_handle)
  }
}

impl Entity {
  virtual_method!(pub fn networkable[4](&self) -> &NetworkableEntity);
  virtual_method!(pub fn renderable[5](&self) -> &RenderableEntity);
  virtual_method!(pub fn abs_origin[9](&self) -> &Vector3D);
  virtual_method!(pub fn spawn[29](&self));
  virtual_method!(pub fn is_alive[131](&self) -> bool);
  virtual_method!(pub fn is_player[132](&self) -> bool);
  virtual_method!(pub fn set_sequence[189](&self, sequence: i32));
  // virtual_method!(pub fn update_client_side_animation[193](&self));
  virtual_method!(pub fn send_viewmodel_matching_sequence[209](&self, sequence: i32));
  virtual_method!(pub fn active_weapon[227](&self) -> Option<&Entity>);
  virtual_method!(pub fn weapon_id[371](&self) -> WeaponId);

  netvar!(pub fn team -> i32 for CBaseEntity->m_iTeamNum);
  netvar!(pub fn owner_handle -> EntityHandle for CBaseCombatWeapon->m_hOwner);
  netvar!(pub fn player_class -> PlayerClass for CCSPlayer->m_iClass);
  netvar!(pub fn sequence -> i32 for CBaseAnimating->m_nSequence);

  netvar!(fn player_spotted -> [bool; 65] for CCSPlayerResource->m_bPlayerSpotted);
  netvar!(fn viewmodel_handle -> EntityHandle for CBasePlayer->m_hViewModel[0]);
  netvar!(fn flags -> EntityFlags for CBasePlayer->m_fFlags);
  netvar!(fn weapon_mode -> WeaponMode for CWeaponCSBase->m_weaponMode);
  netvar!(fn observer_target_handle -> EntityHandle for CBasePlayer->m_hObserverTarget);
}

#[repr(C)]
pub struct NetworkableEntity;

impl NetworkableEntity {
  virtual_method!(pub fn release[1](&self));
  virtual_method!(pub fn client_class[2](&self) -> &ClientClass);
  virtual_method!(pub fn on_data_changed[5](&self, update_kind: i32));
  virtual_method!(pub fn is_dormant[8](&self) -> bool);
  virtual_method!(pub fn index[9](&self) -> i32);
}

#[repr(C)]
pub struct RenderableEntity;

impl RenderableEntity {
  virtual_method!(pub fn should_draw[3](&self) -> bool);
  virtual_method!(pub fn model[9](&self) -> Option<&'static Model>);
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
  Knife,
}

impl WeaponId {
  pub fn from_weapon_name(s: &str) -> Option<Self> {
    Some(match s {
      "ak47" => WeaponId::AK47,
      "knife" => WeaponId::Knife,
      _ => return None,
    })
  }
}

#[derive(Clone, Copy)]
#[open_enum]
#[repr(C)]
enum WeaponMode {
  Secondary = 1,
}

#[derive(Clone, Copy)]
#[open_enum]
#[repr(C)]
pub enum PlayerClass {
  PhoenixConnection = 1,
  LeetKrew,
  ArcticAvengers,
  GuerillaWarfare,
  SealTeam6,
  GSG9,
  Sas,
  Gign,
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
