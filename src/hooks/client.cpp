#include "hooks.h"

#include <app.h>

#include <internal/convar.h>

#include <interfaces/cvar.h>

void __stdcall hooks::frame_stage_notify(int stage) {
  App::with([=](App &app) {
    app.cvar->find_var("mat_postprocess_enable")->set_value(0);
    app.client_vmt.call_original<void, 36>(stage);
  });
}
