use crate::App;

use dopamine_sdk::surface::Surface;

pub extern "C" fn is_cursor_visible(this: &Surface) -> bool {
  App::with_mut(move |app| (app.hooks.is_cursor_visible.original)(this) || app.menu.is_open())
}

pub extern "C" fn lock_cursor(this: &Surface) {
  App::with_mut(move |app| {
    if app.menu.is_open() {
      this.unlock_cursor();
    } else {
      (app.hooks.lock_cursor.original)(this);
    }
  })
}
