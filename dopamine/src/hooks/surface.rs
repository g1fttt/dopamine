use crate::App;

use dopamine_sdk::game::surface::Surface;

pub extern "thiscall" fn is_cursor_visible(this: &Surface) -> bool {
  App::with(move |app| (app.hooks.is_cursor_visible.original)(this) || app.menu.is_open())
}

pub extern "thiscall" fn lock_cursor(this: &Surface) {
  App::with(move |app| {
    if app.menu.is_open() {
      this.unlock_cursor();
    } else {
      (app.hooks.lock_cursor.original)(this);
    }
  })
}
