use crate::app::App;
use crate::features::visuals;

use dopamine_sdk::math::{Angles, Vector3D};
use dopamine_sdk::{Entity, RecvPropProxyData};

use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

use std::ffi::c_void;

pub extern "fastcall" fn calc_viewmodel_view(
  this: &Entity,
  owner: &Entity,
  eye_origin: &Vector3D,
  eye_angles: &Angles,
) {
  App::with_mut(move |app| {
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

pub extern "fastcall" fn should_flip_viewmodel(_this: &Entity) -> bool {
  App::with_mut(|app| !app.config.model_changer.enabled)
}

// NOTE: Doesn't work yet
pub extern "C" fn on_sequence_change(
  data: &mut RecvPropProxyData,
  r#struct: *mut c_void,
  _out: *mut c_void,
) {
  let mut sequence = unsafe { data.value.int };

  let viewmodel = unsafe { &*r#struct.cast_const().cast::<Entity>() };

  let lookat01 = viewmodel.lookup_sequence("lookat01");

  if lookat01 != -1 {
    if sequence == lookat01 {
      return;
    }

    unsafe {
      if GetAsyncKeyState(0x46 /* F */) & 1 != 0 {
        sequence = lookat01;
      }
    }
  }

  viewmodel.set_sequence(sequence);
}
