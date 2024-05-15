use crate::game::render_view::ColorModulation;

use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Color(f32, f32, f32, f32);

impl Color {
    pub const WHITE: Self = Self(1.0, 1.0, 1.0, 1.0);

    pub fn color_modulation(&self) -> ColorModulation {
        (self.0, self.1, self.2)
    }

    pub fn alpha(&self) -> f32 {
        self.3
    }
}
