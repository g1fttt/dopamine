#include "hooks.h"

#include <app.h>
#include <menu.h>

#include <interfaces/surface.h>

void STDCALL hooks::lock_cursor() {
  App::with([](const App &app) {
    if (core::Menu::get().is_open()) {
      return app.interfaces.surface->unlock_cursor();
    }
    app.vmts.surface.call_original<void, 62>();
  });
}
