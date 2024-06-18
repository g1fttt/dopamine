use crate::config::SniperRifleCrosshair;
use crate::game::Entity;

use imgui::{DrawListMut, ImColor32, Io};

pub fn draw_sniper_rifle_crosshair(
  config: &SniperRifleCrosshair,
  local_player: Option<&Entity>,
  io: &Io,
  draw_list: DrawListMut,
) {
  if !config.enabled {
    return;
  }

  let should_draw = local_player
    .and_then(Entity::active_weapon)
    .map(|wp| wp.is_sniper_rifle() && !wp.is_in_scope())
    .unwrap_or_default();

  if !should_draw {
    return;
  }

  let (display_width, display_height) = io.display_size.into();
  let (horiz_center, vert_center) = (display_width / 2.0, display_height / 2.0);

  let im_color = ImColor32::from_rgba_f32s(
    config.color.r,
    config.color.g,
    config.color.b,
    config.color.a,
  );

  // Vertical
  draw_list
    .add_rect(
      [horiz_center - config.thickness, vert_center - config.size],
      [horiz_center + config.thickness, vert_center + config.size],
      im_color,
    )
    .filled(true)
    .build();

  // Horizontal
  draw_list
    .add_rect(
      [horiz_center - config.size, vert_center - config.thickness],
      [horiz_center + config.size, vert_center + config.thickness],
      im_color,
    )
    .filled(true)
    .build();
}
