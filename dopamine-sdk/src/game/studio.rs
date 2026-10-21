use crate::Hitbox;
use crate::math::Vec3;

pub const MAX_STUDIO_BONES: i32 = 128;

#[repr(C)]
pub struct StudioHeader {
  pad: [u8; 176],
  hitbox_set_index: i32,
}

impl StudioHeader {
  pub fn hitbox_set(&self, index: usize) -> Option<&StudioHitboxSet> {
    unsafe {
      (self as *const Self)
        .byte_offset(self.hitbox_set_index as isize)
        .cast::<StudioHitboxSet>()
        .add(index)
        .as_ref()
    }
  }
}

#[repr(C)]
pub struct StudioHitboxSet {
  pad: [u8; 8],
  hitbox_index: i32,
}

impl StudioHitboxSet {
  pub fn hitbox(&self, hitbox: Hitbox) -> Option<&StudioBoundingBox> {
    let bbox = unsafe {
      (self as *const Self)
        .byte_offset(self.hitbox_index as isize)
        .cast::<StudioBoundingBox>()
        .add(hitbox.into())
        .as_ref()
    };

    // Common during level initialization process due to garbage in memory
    if bbox.is_some_and(|b| b.bone_index >= MAX_STUDIO_BONES) {
      return None;
    }

    bbox
  }
}

#[repr(C)]
pub struct StudioBoundingBox {
  pub bone_index: i32,
  pad1: [u8; 4],
  pub mins: Vec3,
  pub maxs: Vec3,
  pad2: [u8; 36],
}

impl From<Hitbox> for usize {
  fn from(val: Hitbox) -> Self {
    val.0 as usize
  }
}
