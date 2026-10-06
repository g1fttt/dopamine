use crate::app::App;
use crate::features::visuals;

use dopamine_sdk::math::{Angles, Vec3};
use dopamine_sdk::{Entity, Hook};

pub extern "C" fn calc_viewmodel_view(
  this: &Entity,
  owner: &Entity,
  eye_origin: &Vec3,
  eye_angles: &Angles,
) {
  App::with_mut(move |app| {
    let original = app.hooks.calc_viewmodel_view.original();
    let original = move |eye_origin| original(this, owner, eye_origin, eye_angles);

    let viewmodel_origin =
      visuals::calc_viewmodel_origin(&app.config.visuals.viewmodel_origin, eye_origin, eye_angles);

    match viewmodel_origin {
      Some(orig) => original(&orig),
      None => original(eye_origin),
    }
  })
}
