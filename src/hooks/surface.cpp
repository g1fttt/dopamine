#include "hooks.h"

#include <app.h>

#include <interfaces/surface.h>

void STDCALL hooks::lock_cursor() {
  App::with([](const App &app) {
    if (app.menu.is_open()) {
      return app.surface->unlock_cursor();
    }
    app.surface_vmt.call_original<void, 62>();
  });
}
