use crate::game::surface::Surface;
use crate::App;

type LockCursorFn = extern "thiscall" fn(&Surface);

pub extern "thiscall" fn lock_cursor(this: &Surface) {
  App::with(move |app| {
    if app.menu.is_open() {
      this.unlock_cursor();
    } else {
      let original: LockCursorFn = app.hooks.lock_cursor.original();
      original(this);
    }
  })
}
