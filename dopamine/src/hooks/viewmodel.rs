use crate::{features::visuals, App};

use dopamine_math::{Angles, Vector};
use dopamine_sdk::game::Entity;

pub extern "thiscall" fn calc_viewmodel_view(
  this: &Entity,
  owner: &Entity,
  eye_origin: &Vector,
  eye_angles: &Angles,
) {
  App::with(move |app| {
    let original = app.hooks.calc_viewmodel_view.original;
    let original = move |eye_origin| original(this, owner, eye_origin, eye_angles);

    let viewmodel_origin =
      visuals::calc_viewmodel_origin(&app.config.visuals.viewmodel_origin, eye_origin, eye_angles);

    match viewmodel_origin {
      Some(orig) => original(&orig),
      None => original(eye_origin),
    }
  })
}
