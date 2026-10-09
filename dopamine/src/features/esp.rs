use crate::config::EnumMapConfig;
use crate::state::{EntityInfo, EntityInfoExtra, GameState, PlayerInfo};

use dopamine_sdk::Color;
use dopamine_sdk::math::{Mat4x4, Vec3};

use enum_map::Enum;
use imgui::{DrawList, ImVec2};
use serde::{Deserialize, Serialize};
use strum::VariantNames;

pub fn draw(config: &EspConfig, draw_list: &mut DrawList, state: &GameState) {
  let Some(view_matrix) = state.view_matrix() else {
    return;
  };

  for entity_info in state.entity_info() {
    let Some(extra_info) = &entity_info.extra_info else {
      continue;
    };

    match extra_info {
      EntityInfoExtra::Player(player_info) => {
        let config_kind =
          if player_info.is_enemy { EspConfigKind::Enemies } else { EspConfigKind::Allies };
        let config = &config[config_kind];

        if let Some(bbox) = BoundingBox::for_player(entity_info, player_info, view_matrix) {
          draw_bounding_box(&config.bounding_box, &bbox, draw_list);

          let health_bar_pos = ImVec2 { x: bbox.mins.x - 7.0, y: bbox.mins.y };
          let health_bar_height = bbox.maxs.y - bbox.mins.y;

          draw_health_info(
            &config.heatlh_bar,
            health_bar_pos,
            health_bar_height,
            player_info.health,
            draw_list,
          );
        }
      }
    };
  }
}

#[derive(Debug)]
struct BoundingBox {
  mins: ImVec2,
  maxs: ImVec2,
}

impl BoundingBox {
  fn for_player(
    entity_info: &EntityInfo,
    player_info: &PlayerInfo,
    view_matrix: &Mat4x4,
  ) -> Option<Self> {
    if !player_info.is_alive || player_info.is_dormant {
      return None;
    }

    let mut screen_min = ImVec2 { x: f32::MAX, y: f32::MAX };
    let mut screen_max = ImVec2 { x: f32::MIN, y: f32::MIN };

    for i in 0..8 {
      let body_point = make_point(&entity_info.obb_mins, &entity_info.obb_maxs, i)
        .transform(&entity_info.coordinate_frame);

      let body_screen_pos = world_to_screen_pixel_aligned(view_matrix, &body_point)?;

      screen_min.x = screen_min.x.min(body_screen_pos.x);
      screen_min.y = screen_min.y.min(body_screen_pos.y);

      screen_max.x = screen_max.x.max(body_screen_pos.x);
      screen_max.y = screen_max.y.max(body_screen_pos.y);

      let head_point = make_point(&player_info.head_obb_mins, &player_info.head_obb_maxs, i)
        .transform(&player_info.head_to_world_transform);

      let head_screen_pos = world_to_screen_pixel_aligned(view_matrix, &head_point)?;

      screen_min.y = screen_min.y.min(head_screen_pos.y);
    }
    Some(Self { mins: screen_min, maxs: screen_max })
  }
}

fn draw_bounding_box(config: &BoundingBoxConfig, bbox: &BoundingBox, draw_list: &mut DrawList) {
  if !config.enabled {
    return;
  }

  let col = &config.color;
  let im_color = imgui::im_col32(col.r, col.g, col.b, col.a);

  draw_list.add_rect(
    ImVec2 { x: bbox.mins.x + 1.0, y: bbox.mins.y + 1.0 },
    ImVec2 { x: bbox.maxs.x + 1.0, y: bbox.maxs.y + 1.0 },
    imgui::im_col32(0.0, 0.0, 0.0, 255.0),
  );

  draw_list.add_rect(bbox.mins, bbox.maxs, im_color);
}

/// Draws a health bar and a health amount at the top point of the health bar
fn draw_health_info(
  config: &HealthBarConfig,
  pos: ImVec2,
  height: f32,
  health: i32,
  draw_list: &mut DrawList,
) {
  if !config.enabled {
    return;
  }

  let width = 3.0;

  let min = pos;
  let max = ImVec2 { x: pos.x + width, y: pos.y + height };

  draw_list.add_rect_filled(
    ImVec2 { x: min.x + 1.0, y: min.y + 1.0 },
    ImVec2 { x: max.x + 1.0, y: max.y + 1.0 },
    imgui::im_col32(0.0, 0.0, 0.0, 255.0),
  );

  let (r, g) = match health {
    65..=100 => (0.0, 255.0),
    30..65 => (255.0, 200.0),
    _ => (255.0, 0.0),
  };

  let min = ImVec2 { x: pos.x, y: pos.y + ((100 - health) as f32 / 100.0 * height) };

  draw_list.add_rect_filled(min, max, imgui::im_col32(r, g, 0.0, 255.0));

  let health_string = health.to_string();
  let text_size = imgui::calc_text_size(&health_string);

  let padding_from_bar = 3.0;
  let pos = ImVec2 { x: min.x - text_size.x - padding_from_bar, y: min.y };

  draw_list.add_text(pos, imgui::im_col32(255.0, 255.0, 255.0, 255.0), health_string);
}

fn make_point(mins: &Vec3, maxs: &Vec3, i: usize) -> Vec3 {
  Vec3 {
    x: if i & 1 > 0 { maxs.x } else { mins.x },
    y: if i & 2 > 0 { maxs.y } else { mins.y },
    z: if i & 4 > 0 { maxs.z } else { mins.z },
  }
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
pub struct HealthBarConfig {
  pub enabled: bool,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EspItemConfig {
  pub bounding_box: BoundingBoxConfig,
  pub heatlh_bar: HealthBarConfig,
}

pub type EspConfig = EnumMapConfig<EspConfigKind, EspItemConfig>;
