use super::material_system::Vec2;

use dopamine_macros::virtual_method;

use std::mem::MaybeUninit;

pub type ColorModulation = (f32, f32, f32);

#[repr(C)]
pub struct RenderView;

impl RenderView {
    pub fn set_color_modulation(&self, (r, g, b): ColorModulation) {
        self.set_color_modulation_private([r, g, b].as_ptr());
    }

    pub fn color_modulation(&self) -> ColorModulation {
        let mut out: [MaybeUninit<f32>; 3] = MaybeUninit::uninit_array();
        self.color_modulation_private(out.as_mut_ptr().cast());
        unsafe { MaybeUninit::array_assume_init(out) }.into()
    }
}

impl RenderView {
    #[virtual_method(index = 4)]
    fn set_blend(&self, blend: f32);

    #[virtual_method(index = 5)]
    fn blend(&self) -> f32;

    #[virtual_method(index = 6, private)]
    fn set_color_modulation_private(&self, color: *const f32);

    #[virtual_method(index = 7, private)]
    fn color_modulation_private(&self, color: *mut f32);
}

#[repr(C)]
pub struct ViewSetup {
    pad1: [u8; 16],
    width: i32,
    pad2: [u8; 4],
    height: i32,
}

impl ViewSetup {
    pub fn dimensions(&self) -> Vec2<i32> {
        (self.width, self.height)
    }
}
