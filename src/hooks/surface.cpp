#include "hooks.h"

#include <game/surface.h>
#include <ui/menu.h>

#include <interfaces.h>

namespace surface {
  bool STDCALL is_cursor_visible() {
    return hooks->is_cursor_visible.call_original() || ui::menu.is_open();
  }

  void STDCALL lock_cursor() {
    return ui::menu.is_open() ? core::interfaces->surface->unlock_cursor()
                              : hooks->lock_cursor.call_original();
  }
}
