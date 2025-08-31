use educe::Educe;
use imgui::{DrawList, ImVec2};
use serde::{Deserialize, Serialize};

use dopamine_sdk::math::{Angles, Vector3D};
use dopamine_sdk::render_view::ViewSetup;
use dopamine_sdk::{Color, Entity};

pub fn draw_better_crosshair(
  local_player: Option<&Entity>,
  config: &BetterCrosshairConfig,
  draw_list: &mut DrawList,
) {
  if !config.enabled {
    return;
  }

  let active_weapon = local_player.and_then(Entity::active_weapon);

  match active_weapon {
    Some(wp) => {
      if wp.is_sniper_rifle() && (!config.force_sniper_rifles || wp.is_in_scope()) {
        return;
      }
    }
    None => return,
  };

  let io = imgui::io();
  let display_width = io.display_size.x;
  let display_height = io.display_size.y;

  let (horiz_center, vert_center) = (display_width / 2.0, display_height / 2.0);

  let col = &config.color;
  let im_color = imgui::im_col32(col.r, col.g, col.b, col.a);

  let size = config.size;
  let thick = config.thickness;
  let gap = config.gap;

  // Up
  draw_list.add_rect_filled(
    ImVec2 { x: horiz_center - thick / 2.0, y: vert_center - size - gap / 2.0 },
    ImVec2 { x: horiz_center + thick / 2.0, y: vert_center - gap / 2.0 },
    im_color,
  );

  // Down
  draw_list.add_rect_filled(
    ImVec2 { x: horiz_center - thick / 2.0, y: vert_center + gap / 2.0 },
    ImVec2 { x: horiz_center + thick / 2.0, y: vert_center + size + gap / 2.0 },
    im_color,
  );

  // Left
  draw_list.add_rect_filled(
    ImVec2 { x: horiz_center - size - gap / 2.0, y: vert_center - thick / 2.0 },
    ImVec2 { x: horiz_center - gap / 2.0, y: vert_center + thick / 2.0 },
    im_color,
  );

  // Right
  draw_list.add_rect_filled(
    ImVec2 { x: horiz_center + gap / 2.0, y: vert_center - thick / 2.0 },
    ImVec2 { x: horiz_center + gap / 2.0 + size, y: vert_center + thick / 2.0 },
    im_color,
  );
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
    eye_origin + (ortho * config.value.x + forward * config.value.y + up * config.value.z);

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
  pub value: Vector3D,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct VisualsConfig {
  pub better_crosshair: BetterCrosshairConfig,
  pub add_fov: AddFovConfig,
  pub viewmodel_origin: ViewmodelOriginConfig,
}
