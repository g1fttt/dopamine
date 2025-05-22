use super::FeatureContext;

use educe::Educe;
use imgui::{DrawListMut, ImColor32, Io};
use serde::{Deserialize, Serialize};

use dopamine_sdk::math::{Angles, Vector3D};
use dopamine_sdk::render_view::ViewSetup;
use dopamine_sdk::{Color, Entity};

pub fn draw_better_crosshair(
  ctx: FeatureContext<'_, '_, BetterCrosshairConfig>,
  io: &Io,
  draw_list: DrawListMut,
) {
  if !ctx.config.enabled {
    return;
  }

  let active_weapon = ctx.local_player.and_then(Entity::active_weapon);

  match active_weapon {
    Some(wp) => {
      if wp.is_sniper_rifle() && (!ctx.config.force_sniper_rifles || wp.is_in_scope()) {
        return;
      }
    }
    None => return,
  };

  let (display_width, display_height) = io.display_size.into();
  let (horiz_center, vert_center) = (display_width / 2.0, display_height / 2.0);

  let col = &ctx.config.color;
  let im_color = ImColor32::from_rgba_f32s(col.r, col.g, col.b, col.a);

  let size = ctx.config.size;
  let thick = ctx.config.thickness;
  let gap = ctx.config.gap;

  // Up
  draw_list
    .add_rect(
      [horiz_center - thick / 2.0, vert_center - size - gap / 2.0],
      [horiz_center + thick / 2.0, vert_center - gap / 2.0],
      im_color,
    )
    .filled(true)
    .build();

  // Down
  draw_list
    .add_rect(
      [horiz_center - thick / 2.0, vert_center + gap / 2.0],
      [horiz_center + thick / 2.0, vert_center + size + gap / 2.0],
      im_color,
    )
    .filled(true)
    .build();

  // Left
  draw_list
    .add_rect(
      [horiz_center - size - gap / 2.0, vert_center - thick / 2.0],
      [horiz_center - gap / 2.0, vert_center + thick / 2.0],
      im_color,
    )
    .filled(true)
    .build();

  // Right
  draw_list
    .add_rect(
      [horiz_center + gap / 2.0, vert_center - thick / 2.0],
      [horiz_center + gap / 2.0 + size, vert_center + thick / 2.0],
      im_color,
    )
    .filled(true)
    .build();
}

#[inline]
pub fn add_fov(config: &AddFovConfig, view: &mut ViewSetup) {
  if config.enabled {
    view.fov += config.amount;
  }
}

pub fn calc_viewmodel_origin(
  config: &ViewmodelOriginConfig,
  eye_origin: &Vector3D,
  eye_angles: &Angles,
) -> Option<Vector3D> {
  if !config.enabled {
    return None;
  }

  let forward = eye_angles.forward_vector();
  let up = eye_angles.up_vector();
  let ortho = forward.cross_product(&up);

  let new_eye_origin =
    eye_origin + (ortho * config.origin.x + forward * config.origin.y + up * config.origin.z);

  Some(new_eye_origin)
}

#[derive(Educe, Serialize, Deserialize)]
#[serde(default)]
#[educe(Default)]
pub struct BetterCrosshairConfig {
  pub enabled: bool,
  pub force_sniper_rifles: bool,
  #[educe(Default = 5.0)]
  pub size: f32,
  #[educe(Default = 1.0)]
  pub thickness: f32,
  #[educe(Default = 1.0)]
  pub gap: f32,
  pub color: Color,
}

#[derive(Educe, Serialize, Deserialize)]
#[serde(default)]
#[educe(Default)]
pub struct AddFovConfig {
  pub enabled: bool,
  #[educe(Default = 10.0)]
  pub amount: f32,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ViewmodelOriginConfig {
  pub enabled: bool,
  #[serde(flatten)]
  pub origin: Vector3D,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct VisualsConfig {
  pub better_crosshair: BetterCrosshairConfig,
  pub add_fov: AddFovConfig,
  pub viewmodel_origin: ViewmodelOriginConfig,
}
