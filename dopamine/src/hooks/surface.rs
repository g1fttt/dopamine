use crate::game::surface::Surface;
use crate::App;

type IsCursorVisibleFn = extern "thiscall" fn(&Surface) -> bool;

pub extern "thiscall" fn is_cursor_visible(this: &Surface) -> bool {
  App::with(move |app| {
    let original: IsCursorVisibleFn = app.hooks.is_cursor_visible.original();
    original(this) || app.menu.is_open()
  })
}

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
