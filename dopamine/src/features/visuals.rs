use super::FeatureContext;

use educe::Educe;
use imgui::{DrawListMut, ImColor32, Io};
use serde::{Deserialize, Serialize};

use dopamine_misc::math::{Angles, Vector};
use dopamine_misc::Color;

use dopamine_sdk::game::render_view::ViewSetup;
use dopamine_sdk::game::Entity;

pub fn draw_no_scope_crosshair(
  ctx: FeatureContext<'_, '_, NoScopeCrosshairConfig>,
  io: &Io,
  draw_list: DrawListMut,
) {
  if !ctx.config.enabled {
    return;
  }

  let should_draw = ctx
    .local_player
    .and_then(Entity::active_weapon)
    .map(|wp| wp.is_sniper_rifle() && !wp.is_in_scope())
    .unwrap_or_default();

  if !should_draw {
    return;
  }

  let (display_width, display_height) = io.display_size.into();
  let (horiz_center, vert_center) = (display_width / 2.0, display_height / 2.0);

  let im_color = ImColor32::from_rgba_f32s(
    ctx.config.color.r,
    ctx.config.color.g,
    ctx.config.color.b,
    ctx.config.color.a,
  );

  // Vertical
  draw_list
    .add_rect(
      [horiz_center - ctx.config.thickness, vert_center - ctx.config.size],
      [horiz_center + ctx.config.thickness, vert_center + ctx.config.size],
      im_color,
    )
    .filled(true)
    .build();

  // Horizontal
  draw_list
    .add_rect(
      [horiz_center - ctx.config.size, vert_center - ctx.config.thickness],
      [horiz_center + ctx.config.size, vert_center + ctx.config.thickness],
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
  eye_origin: &Vector,
  eye_angles: &Angles,
) -> Option<Vector> {
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
pub struct NoScopeCrosshairConfig {
  pub enabled: bool,
  #[educe(Default = 5.0)]
  pub size: f32,
  #[educe(Default = 1.0)]
  pub thickness: f32,
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
  pub origin: Vector,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct VisualsConfig {
  pub no_scope_crosshair: NoScopeCrosshairConfig,
  pub add_fov: AddFovConfig,
  pub viewmodel_origin: ViewmodelOriginConfig,
}
