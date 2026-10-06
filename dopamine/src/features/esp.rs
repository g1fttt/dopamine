use crate::config::EnumMapConfig;
use crate::state::{EntityInfo, GameState};

use dopamine_sdk::Color;
use dopamine_sdk::math::{Mat4x4, Vec3};

use enum_map::Enum;
use imgui::{AddRectBuilder, DrawList, ImVec2};
use serde::{Deserialize, Serialize};
use strum::VariantNames;

pub fn draw(config: &EspConfig, draw_list: &mut DrawList, state: &GameState) {
  let Some(matrix) = state.view_matrix() else {
    return;
  };

  for info in state.entity_info() {
    if !info.is_player || !info.is_alive {
      continue;
    }

    let config_kind = if info.is_enemy { EspConfigKind::Enemies } else { EspConfigKind::Allies };
    let item_config = &config[config_kind];

    draw_bounding_box(&item_config.bounding_box, draw_list, info, matrix);
  }
}

fn draw_bounding_box(
  config: &BoundingBoxConfig,
  draw_list: &mut DrawList,
  info: &EntityInfo,
  matrix: &Mat4x4,
) {
  if !config.enabled {
    return;
  }

  let mut screen_min = ImVec2 { x: f32::MAX, y: f32::MAX };
  let mut screen_max = ImVec2 { x: f32::MIN, y: f32::MIN };

  fn make_point(mins: &Vec3, maxs: &Vec3, i: usize) -> Vec3 {
    Vec3 {
      x: if i & 1 > 0 { maxs.x } else { mins.x },
      y: if i & 2 > 0 { maxs.y } else { mins.y },
      z: if i & 4 > 0 { maxs.z } else { mins.z },
    }
  }

  for i in 0..8 {
    let body_point =
      make_point(&info.obb_mins, &info.obb_maxs, i).transform(&info.coordinate_frame);

    let Some(body_screen_pos) = world_to_screen_pixel_aligned(matrix, &body_point) else {
      return;
    };

    screen_min.x = screen_min.x.min(body_screen_pos.x);
    screen_min.y = screen_min.y.min(body_screen_pos.y);

    screen_max.x = screen_max.x.max(body_screen_pos.x);
    screen_max.y = screen_max.y.max(body_screen_pos.y);

    let head_point =
      make_point(&info.head_obb_mins, &info.head_obb_maxs, i).transform(&info.head_to_world_matrix);

    let Some(head_screen_pos) = world_to_screen_pixel_aligned(matrix, &head_point) else {
      return;
    };

    screen_min.y = screen_min.y.min(head_screen_pos.y);
  }

  let col = &config.color;
  let im_color = imgui::im_col32(col.r, col.g, col.b, col.a);

  draw_list.add_rect(
    ImVec2 { x: screen_min.x + 1.0, y: screen_min.y + 1.0 },
    ImVec2 { x: screen_max.x + 1.0, y: screen_max.y + 1.0 },
    imgui::im_col32(0.0, 0.0, 0.0, 255.0),
  );

  AddRectBuilder::default().min(screen_min).max(screen_max).color(im_color).build(draw_list);
}

#[rustfmt::skip]
fn world_pos_to_screen_pos(matrix: &Mat4x4, world_pos: &Vec3) -> Option<ImVec2> {
  let w = matrix[3][0] * world_pos.x + matrix[3][1] * world_pos.y + matrix[3][2] * world_pos.z + matrix[3][3];
  if w < 0.001 {
    return None;
  }

  let mut screen_pos = imgui::io().display_size;
  screen_pos.x /= 2.0;
  screen_pos.y /= 2.0;

  screen_pos.x *= 1.0 + (matrix[0][0] * world_pos.x + matrix[0][1] * world_pos.y + matrix[0][2] * world_pos.z + matrix[0][3]) / w;
  screen_pos.y *= 1.0 - (matrix[1][0] * world_pos.x + matrix[1][1] * world_pos.y + matrix[1][2] * world_pos.z + matrix[1][3]) / w;

  Some(screen_pos)
}

fn world_to_screen_pixel_aligned(matrix: &Mat4x4, world_pos: &Vec3) -> Option<ImVec2> {
  world_pos_to_screen_pos(matrix, world_pos)
    .map(|pos| ImVec2 { x: pos.x.floor(), y: pos.y.floor() })
}

#[derive(Clone, Copy, Enum, VariantNames, Serialize, Deserialize)]
pub enum EspConfigKind {
  Enemies,
  Allies,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BoundingBoxConfig {
  pub enabled: bool,
  pub color: Color,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EspItemConfig {
  pub bounding_box: BoundingBoxConfig,
}

pub type EspConfig = EnumMapConfig<EspConfigKind, EspItemConfig>;
