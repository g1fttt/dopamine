#include "hooks.h"

#include <app.h>

#include <internal/convar.h>

#include <interfaces/cvar.h>

void STDCALL hooks::frame_stage_notify(int32_t stage) {
  App::with([=](const App &app) {
    app.cvar->find_var("mat_postprocess_enable")->set_value(0);
    app.client_vmt.call_original<void, 36>(stage);
  });
}
