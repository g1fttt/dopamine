#include "hooks.h"

#include <app.h>

#include <interfaces/surface.h>
#include <ui/menu.h>

void STDCALL hooks::lock_cursor() {
  App::with<void>([](const App &app) {
    if (ui::Menu::get().is_open()) {
      return app.interfaces.surface->unlock_cursor();
    }
    app.vmts.surface.call_original<void, 62>();
  });
}
