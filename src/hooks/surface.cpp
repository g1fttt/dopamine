#include "hooks.h"

#include <interfaces/surface.h>
#include <ui/menu.h>

#include <app.h>

void STDCALL hooks::lock_cursor() {
  App::with<void>([](const App &app) {
    return ui::Menu::get().is_open()
               ? app.interfaces.surface->unlock_cursor()
               : app.vmts.surface.call_original<void, 62>();
  });
}
