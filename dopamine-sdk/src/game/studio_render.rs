use crate::material_system::Material;

#[repr(C)]
pub struct StudioRender<'a> {
  pad: [u8; 608],
  forced_material: Option<&'a Material>,
}

impl StudioRender<'_> {
  #[inline]
  pub fn is_material_overrided(&self) -> bool {
    self.forced_material.is_some()
  }
}

unsafe impl Send for StudioRender<'_> {}
unsafe impl Sync for StudioRender<'_> {}
