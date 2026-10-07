use crate::engine::Model;
use crate::game::{ClassId, ClientClass};
use crate::interfaces::{engine, entity_list, model_info};
use crate::math::{Mat3x4, Vec3};
use crate::utils::{Netvars, Patterns};
use crate::{netvar, virtual_method};

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

  pub fn is_viewmodel(&self) -> bool {
    self.networkable().client_class().id == ClassId::PredictedViewModel
  }

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

  /// Retrieves OBB minimum, OBB maximum and "transform to world" matrix by utilizing hitbox bone
  pub fn bone_info(&self, hitbox: Hitbox) -> Option<(Vec3, Vec3, &Mat3x4)> {
    let model = self.renderable().model()?;
    let studio_header = model_info().studio_header(model)?;

    let hitbox_set = studio_header.hitbox_set(self.hitbox_set() as usize)?;
    let head_hitbox = hitbox_set.hitbox(hitbox)?;

    let bones = self.bone_accessor()?;
    let matrix = bones.to_world_transform(head_hitbox.bone_index as usize)?;

    Some((head_hitbox.mins, head_hitbox.maxs, matrix))
  }

  pub fn bone_accessor(&self) -> Option<&BoneAccessor> {
    let force_bone = Netvars::get().get(&("CBaseAnimating", "m_nForceBone"))?;

    unsafe {
      // 48 8B C4 4C 89 48 ? 4C 89 40 ? 55 53 41 57
      // (*(*m_pRagdoll + 8i64))(m_pRagdoll, this, pbones, *(*hdr + 156i64), boneSimulated, this + 257);
      // ------------------------------------------------------------------------------------------^^^
      (self as *const Self).byte_add(force_bone.offset + 20).cast::<BoneAccessor>().as_ref()
    }
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
  virtual_method!(pub fn collideable[3](&self) -> &CollideableEntity);
  virtual_method!(pub fn networkable[4](&self) -> &NetworkableEntity);
  virtual_method!(pub fn renderable[5](&self) -> &RenderableEntity);
  virtual_method!(pub fn abs_origin[9](&self) -> &Vec3);
  virtual_method!(pub fn is_alive[131](&self) -> bool);
  virtual_method!(pub fn is_player[132](&self) -> bool);
  virtual_method!(pub fn active_weapon[227](&self) -> Option<&Entity>);
  virtual_method!(pub fn weapon_id[371](&self) -> WeaponId);

  netvar!(pub fn health -> i32 as CBasePlayer->m_iHealth);
  netvar!(pub fn team -> i32 as CBaseEntity->m_iTeamNum);
  netvar!(pub fn owner_handle -> EntityHandle as CBaseCombatWeapon->m_hOwner);
  netvar!(pub fn hitbox_set -> i32 as CBaseAnimating->m_nHitboxSet);
  netvar!(fn player_spotted -> [bool; 65] as CCSPlayerResource->m_bPlayerSpotted);
  netvar!(fn flags -> EntityFlags as CBasePlayer->m_fFlags);
  netvar!(fn weapon_mode -> WeaponMode as CWeaponCSBase->m_weaponMode);
}

#[derive(Clone, Copy)]
#[open_enum]
#[repr(C)]
pub enum Hitbox {
  Head = 12,
}

#[repr(C)]
pub struct BoneAccessor {
  pad: [u8; 8],
  bones: *const Mat3x4,
}

impl BoneAccessor {
  pub fn to_world_transform(&self, index: usize) -> Option<&Mat3x4> {
    unsafe { self.bones.add(index).as_ref() }
  }
}

#[repr(C)]
pub struct CollideableEntity;

impl CollideableEntity {
  virtual_method!(pub fn obb_mins[3](&self) -> &Vec3);
  virtual_method!(pub fn obb_maxs[4](&self) -> &Vec3);
}

#[repr(C)]
pub struct NetworkableEntity;

impl NetworkableEntity {
  virtual_method!(pub fn release[1](&self));
  virtual_method!(pub fn client_class<'a>[2](&self) -> &'a ClientClass<'_>);
  virtual_method!(pub fn is_dormant[8](&self) -> bool);
  virtual_method!(pub fn index[9](&self) -> i32);
}

#[repr(C)]
pub struct RenderableEntity;

impl RenderableEntity {
  pub fn base(&self) -> Option<&Entity> {
    self.unknown_entity().base_entity()
  }
}

impl RenderableEntity {
  virtual_method!(fn unknown_entity<'a>[0](&self) -> &'a UnknownEntity);
  virtual_method!(pub fn should_draw[3](&self) -> bool);
  virtual_method!(pub fn model[9](&self) -> Option<&Model>);
  virtual_method!(pub fn draw_model[10](&self) -> i32 where (i32: 1 /* StudioRender */));
  // 48 89 5C 24 ? 55 56 41 56 48 81 EC ? ? ? ? 49 8B 00
  // ...
  // coordinateFrame = (*(*i + 272i64))(i);
  // --------------------------^^^ => 272 / 8 = 34
  // ConcatTransforms(a2, coordinateFrame, v19);
  // TransformAABB(v19, v18, v17, v15, v16);
  // ...
  virtual_method!(pub fn to_world_transform[34](&self) -> &Mat3x4);
}

#[repr(C)]
struct UnknownEntity;

impl UnknownEntity {
  virtual_method!(fn base_entity<'a>[7](&self) -> Option<&'a Entity>);
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
