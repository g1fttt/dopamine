#include "hooks.h"

#include <interfaces/surface.h>
#include <ui/menu.h>

#include <app.h>

using ui::Menu;

bool STDCALL hooks::is_cursor_visible() {
  return App::get().vmts.surface.call_original<bool, 53>() ||
         Menu::get().is_open();
}

void STDCALL hooks::lock_cursor() {
  App::get().and_then<void>([](const App &app) {
    return Menu::get().is_open() ? app.interfaces.surface->unlock_cursor()
                                 : app.vmts.surface.call_original<void, 62>();
  });
}
