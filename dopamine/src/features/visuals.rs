use crate::config::{AddFov, NoScopeCrosshairConfig};
use crate::game::render_view::ViewSetup;
use crate::game::Entity;

use imgui::{DrawListMut, ImColor32, Io};

use super::FeatureContext;

pub fn draw_sniper_crosshair(
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
pub fn add_fov(config: &AddFov, view: &mut ViewSetup) {
  if config.enabled {
    view.fov += config.amount;
  }
}
