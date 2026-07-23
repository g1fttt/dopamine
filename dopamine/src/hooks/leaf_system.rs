use crate::app::App;

use dopamine_sdk::math::Vector3D;
use dopamine_sdk::{Hook, RenderableEntity};

pub extern "C" fn calc_renderable_world_space_aabb_fast(
  renderable: &RenderableEntity,
  abs_min: &mut Vector3D,
  abs_max: &mut Vector3D,
) {
  App::with_mut(|app| {
    if renderable.base().is_none_or(|ent| !ent.is_player())
      || !app.config.misc.disable_model_occlusion
    {
      return (app.hooks.calc_renderable_world_space_aabb_fast.original())(
        renderable, abs_min, abs_max,
      );
    }

    const MAX_COORD: f32 = 16384.0;
    const MIN_COORD: f32 = -MAX_COORD;

    const MAX_COORD_3D: Vector3D = Vector3D::new(MAX_COORD, MAX_COORD, MAX_COORD);
    const MIN_COORD_3D: Vector3D = Vector3D::new(MIN_COORD, MIN_COORD, MIN_COORD);

    *abs_max = MAX_COORD_3D;
    *abs_min = MIN_COORD_3D;
  })
}
