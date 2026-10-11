use crate::app::App;
use crate::features::visuals;

use dopamine_sdk::math::Vec3;
use dopamine_sdk::{Hook, RenderableEntity};

pub extern "C" fn calc_renderable_world_space_aabb_fast(
  renderable: &RenderableEntity,
  abs_min: &mut Vec3,
  abs_max: &mut Vec3,
) {
  App::with_mut(|app| {
    let Some((max, min)) = visuals::extend_player_world_space_aabb(
      app.config.visuals.disable_model_occlusion,
      renderable,
    ) else {
      return (app.hooks.calc_renderable_world_space_aabb_fast.original())(
        renderable, abs_min, abs_max,
      );
    };

    *abs_max = max;
    *abs_min = min;
  })
}
